use super::*;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Runestone {
  pub edicts: Vec<Edict>,
  pub etching: Option<Etching>,
  pub mint: Option<RuneId>,
  pub pointer: Option<u32>,
}

#[derive(Debug, PartialEq)]
enum Payload {
  Valid(Vec<u8>),
  Invalid(Flaw),
}

impl Runestone {
  pub const MAGIC_NUMBER: opcodes::Opcode = opcodes::all::OP_PUSHNUM_13;
  pub const COMMIT_CONFIRMATIONS: u16 = 6;

  pub fn decipher(output_len: u32, payload: Vec<u8>) -> Option<Artifact> {
    let Ok(integers) = Runestone::integers(&payload) else {
      return Some(Artifact::Cenotaph(Cenotaph {
        flaw: Some(Flaw::Varint),
        ..default()
      }));
    };

    let Message {
      mut flaw,
      edicts,
      mut fields,
    } = Message::from_integers(output_len, &integers);

    let mut flags = Tag::Flags
      .take(&mut fields, |[flags]| Some(flags))
      .unwrap_or_default();

    let etching = Flag::Etching.take(&mut flags).then(|| Etching {
      divisibility: Tag::Divisibility.take(&mut fields, |[divisibility]| {
        let divisibility = u8::try_from(divisibility).ok()?;
        (divisibility <= Etching::MAX_DIVISIBILITY).then_some(divisibility)
      }),
      premine: Tag::Premine.take(&mut fields, |[premine]| Some(premine)),
      rune: Tag::Rune.take(&mut fields, |[rune]| Some(Rune(rune))),
      spacers: Tag::Spacers.take(&mut fields, |[spacers]| {
        let spacers = u32::try_from(spacers).ok()?;
        (spacers <= Etching::MAX_SPACERS).then_some(spacers)
      }),
      symbol: Tag::Symbol.take(&mut fields, |[symbol]| {
        char::from_u32(u32::try_from(symbol).ok()?)
      }),
      terms: Flag::Terms.take(&mut flags).then(|| Terms {
        cap: Tag::Cap.take(&mut fields, |[cap]| Some(cap)),
        height: (
          Tag::HeightStart.take(&mut fields, |[start_height]| {
            u64::try_from(start_height).ok()
          }),
          Tag::HeightEnd.take(&mut fields, |[start_height]| {
            u64::try_from(start_height).ok()
          }),
        ),
        amount: Tag::Amount.take(&mut fields, |[amount]| Some(amount)),
        offset: (
          Tag::OffsetStart.take(&mut fields, |[start_offset]| {
            u64::try_from(start_offset).ok()
          }),
          Tag::OffsetEnd.take(&mut fields, |[end_offset]| u64::try_from(end_offset).ok()),
        ),
      }),
      turbo: Flag::Turbo.take(&mut flags),
    });

    let mint = Tag::Mint.take(&mut fields, |[block, tx]| {
      RuneId::new(block.try_into().ok()?, tx.try_into().ok()?)
    });

    let pointer = Tag::Pointer.take(&mut fields, |[pointer]| {
      let pointer = u32::try_from(pointer).ok()?;
      (u64::from(pointer) < u64::try_from(output_len).unwrap()).then_some(pointer)
    });

    if etching
      .map(|etching| etching.supply().is_none())
      .unwrap_or_default()
    {
      flaw.get_or_insert(Flaw::SupplyOverflow);
    }

    if flags != 0 {
      flaw.get_or_insert(Flaw::UnrecognizedFlag);
    }

    if fields.keys().any(|tag| tag % 2 == 0) {
      flaw.get_or_insert(Flaw::UnrecognizedEvenTag);
    }

    if let Some(flaw) = flaw {
      return Some(Artifact::Cenotaph(Cenotaph {
        flaw: Some(flaw),
        mint,
        etching: etching.and_then(|etching| etching.rune),
      }));
    }

    Some(Artifact::Runestone(Self {
      edicts,
      etching,
      mint,
      pointer,
    }))
  }

  fn integers(payload: &[u8]) -> Result<Vec<u128>, varint::Error> {
    let mut integers = Vec::new();
    let mut i = 0;

    while i < payload.len() {
      let (integer, length) = varint::decode(&payload[i..])?;
      integers.push(integer);
      i += length;
    }

    Ok(integers)
  }
}

