// SPDX-License-Identifier: CC0-1.0

use super::*;

fn scriptint_parse(v: &[u8]) -> i64 {
  let (mut ret, sh) = v
    .iter()
    .fold((0, 0), |(acc, sh), n| (acc + ((*n as i64) << sh), sh + 8));
  if v[v.len() - 1] & 0x80 != 0 {
    ret &= (1 << (sh - 1)) - 1;
    ret = -ret;
  }
  ret
}

fn read_scriptint_non_minimal(v: &[u8]) -> Result<i64, Error> {
  if v.is_empty() {
    return Ok(0);
  }
  if v.len() > 4 {
    return Err(Error::NumericOverflow);
  }

  Ok(scriptint_parse(v))
}

fn reserved_len_for_slice(len: usize) -> usize {
  len + match len {
    0..=0x4b => 1,
    0x4c..=0xff => 2,
    0x100..=0xffff => 3,
    // we don't care about oversized, the other fn will panic anyway
    _ => 5,
  }
}

enum UintError {
    EarlyEndOfScript,
    NumericOverflow,
}

pub enum Error {
  EarlyEndOfScript,
  NumericOverflow,
  NonMinimalPush,
}

impl From<UintError> for Error {
  fn from(error: UintError) -> Self {
    match error {
      UintError::EarlyEndOfScript => Error::EarlyEndOfScript,
      UintError::NumericOverflow => Error::NumericOverflow,
    }
  }
}

fn read_uint_iter(data: &mut core::slice::Iter<'_, u8>, size: usize) -> Result<usize, UintError> {
  if data.len() < size {
    Err(UintError::EarlyEndOfScript)
  } else if size > usize::from(u16::max_value() / 8) {
    // Casting to u32 would overflow
    Err(UintError::NumericOverflow)
  } else {
    let mut ret = 0;
    for (i, item) in data.take(size).enumerate() {
      ret = usize::from(*item)
        // Casting is safe because we checked above to not repeat the same check in a loop
        .checked_shl((i * 8) as u32)
        .ok_or(UintError::NumericOverflow)?
        .checked_add(ret)
        .ok_or(UintError::NumericOverflow)?;
    }
    Ok(ret)
  }
}

/// A "parsed opcode" which allows iterating over a [`Script`] in a more sensible way.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Instruction<'a> {
  /// Push a bunch of data.
  PushBytes(&'a PushBytes),
  /// Some non-push opcode.
  Op(Opcode),
}

impl<'a> Instruction<'a> {
  /// Returns the opcode if the instruction is not a data push.
  pub fn opcode(&self) -> Option<Opcode> {
      match self {
          Instruction::Op(op) => Some(*op),
          Instruction::PushBytes(_) => None,
      }
  }

  /// Returns the pushed bytes if the instruction is a data push.
  pub fn push_bytes(&self) -> Option<&PushBytes> {
      match self {
          Instruction::Op(_) => None,
          Instruction::PushBytes(bytes) => Some(bytes),
      }
  }

  /// Returns the number interpretted by the script parser
  /// if it can be coerced into a number.
  ///
  /// This does not require the script num to be minimal.
  pub fn script_num(&self) -> Option<i64> {
    match self {
      Instruction::Op(op) => {
        let v = op.to_u8();
        match v {
          // OP_PUSHNUM_1 ..= OP_PUSHNUM_16
          0x51..=0x60 => Some(v as i64 - 0x50),
          // OP_PUSHNUM_NEG1
          0x4f => Some(-1),
          _ => None,
        }
      }
      Instruction::PushBytes(bytes) => match read_scriptint_non_minimal(bytes.as_bytes()) {
        Ok(v) => Some(v),
        _ => None,
      },
    }
  }

  /// Returns the number of bytes required to encode the instruction in script.
  pub(super) fn script_serialized_len(&self) -> usize {
    match self {
      Instruction::Op(_) => 1,
      Instruction::PushBytes(bytes) => reserved_len_for_slice(bytes.len()),
    }
  }
}

/// Iterator over a script returning parsed opcodes.
#[derive(Debug, Clone)]
pub struct Instructions<'a> {
  pub(crate) data: core::slice::Iter<'a, u8>,
  pub(crate) enforce_minimal: bool,
}

impl<'a> Instructions<'a> {
  /// Views the remaining script as a slice.
  ///
  /// This is analogous to what [`core::str::Chars::as_str`] does.
  // !!!!!!!!
  // pub fn as_script(&self) -> &'a Script { Script::from_bytes(self.data.as_slice()) }

  /// Sets the iterator to end so that it won't iterate any longer.
  pub(super) fn kill(&mut self) {
      let len = self.data.len();
      self.data.nth(len.max(1) - 1);
  }

  /// Takes a `len` bytes long slice from iterator and returns it, advancing the iterator.
  ///
  /// If the iterator is not long enough [`Error::EarlyEndOfScript`] is returned and the iterator
  /// is killed to avoid returning an infinite stream of errors.
  pub(super) fn take_slice_or_kill(&mut self, len: u32) -> Result<&'a PushBytes, Error> {
      let len = len as usize;
      if self.data.len() >= len {
          let slice = &self.data.as_slice()[..len];
          if len > 0 {
              self.data.nth(len - 1);
          }

          Ok(slice
              .try_into()
              .expect("len was created from u32, so can't happen"))
      } else {
          self.kill();
          Err(Error::EarlyEndOfScript)
      }
  }

  pub(super) fn next_push_data_len(
      &mut self,
      len: PushDataLenLen,
      min_push_len: usize,
  ) -> Option<Result<Instruction<'a>, Error>> {
      let n = match read_uint_iter(&mut self.data, len as usize) {
          Ok(n) => n,
          // We do exhaustive matching to not forget to handle new variants if we extend
          // `UintError` type.
          // Overflow actually means early end of script (script is definitely shorter
          // than `usize::MAX`)
          Err(UintError::EarlyEndOfScript) | Err(UintError::NumericOverflow) => {
              self.kill();
              return Some(Err(Error::EarlyEndOfScript));
          }
      };
      if self.enforce_minimal && n < min_push_len {
          self.kill();
          return Some(Err(Error::NonMinimalPush));
      }
      let result = n
          .try_into()
          .map_err(|_| Error::NumericOverflow)
          .and_then(|n| self.take_slice_or_kill(n))
          .map(Instruction::PushBytes);
      Some(result)
  }
}

/// Allowed length of push data length.
///
/// This makes it easier to prove correctness of `next_push_data_len`.
pub(super) enum PushDataLenLen {
    One = 1,
    Two = 2,
    Four = 4,
}

impl<'a> Iterator for Instructions<'a> {
    type Item = Result<Instruction<'a>, Error>;

    fn next(&mut self) -> Option<Result<Instruction<'a>, Error>> {
        let &byte = self.data.next()?;

        // classify parameter does not really matter here since we are only using
        // it for pushes and nums
        match Opcode::from(byte).classify(opcodes::ClassifyContext::Legacy) {
            opcodes::Class::PushBytes(n) => {
                // make sure safety argument holds across refactorings
                let n: u32 = n;

                let op_byte = self.data.as_slice().first();
                match (self.enforce_minimal, op_byte, n) {
                    (true, Some(&op_byte), 1)
                        if op_byte == 0x81 || (op_byte > 0 && op_byte <= 16) =>
                    {
                        self.kill();
                        Some(Err(Error::NonMinimalPush))
                    }
                    (_, None, 0) => {
                        // the iterator is already empty, may as well use this information to avoid
                        // whole take_slice_or_kill function
                        Some(Ok(Instruction::PushBytes(PushBytes::empty())))
                    }
                    _ => Some(self.take_slice_or_kill(n).map(Instruction::PushBytes)),
                }
            }
            opcodes::Class::Ordinary(opcodes::Ordinary::OP_PUSHDATA1) => {
                self.next_push_data_len(PushDataLenLen::One, 76)
            }
            opcodes::Class::Ordinary(opcodes::Ordinary::OP_PUSHDATA2) => {
                self.next_push_data_len(PushDataLenLen::Two, 0x100)
            }
            opcodes::Class::Ordinary(opcodes::Ordinary::OP_PUSHDATA4) => {
                self.next_push_data_len(PushDataLenLen::Four, 0x10000)
            }
            // Everything else we can push right through
            _ => Some(Ok(Instruction::Op(Opcode::from(byte)))),
        }
    }
}
