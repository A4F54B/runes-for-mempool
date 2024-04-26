use super::*;

#[derive(
  Debug,
  PartialEq,
  Copy,
  Clone,
  Hash,
  Eq,
  Ord,
  PartialOrd,
  Default,
  DeserializeFromStr,
  SerializeDisplay,
)]
pub struct RuneId {
  pub block: u64,
  pub tx: u32,
}

impl RuneId {
  pub fn new(block: u64, tx: u32) -> Option<RuneId> {
    let id = RuneId { block, tx };

    if id.block == 0 && id.tx > 0 {
      return None;
    }

    Some(id)
  }

  pub fn delta(self, next: RuneId) -> Option<(u128, u128)> {
    let block = next.block.checked_sub(self.block)?;

    let tx = if block == 0 {
      next.tx.checked_sub(self.tx)?
    } else {
      next.tx
    };

    Some((block.into(), tx.into()))
  }

  pub fn next(self: RuneId, block: u128, tx: u128) -> Option<RuneId> {
    RuneId::new(
      self.block.checked_add(block.try_into().ok()?)?,
      if block == 0 {
        self.tx.checked_add(tx.try_into().ok()?)?
      } else {
        tx.try_into().ok()?
      },
    )
  }
}

impl Display for RuneId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.block, self.tx)
  }
}

impl FromStr for RuneId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (height, index) = s.split_once(':').ok_or(Error::Separator)?;

    Ok(Self {
      block: height.parse().map_err(Error::Block)?,
      tx: index.parse().map_err(Error::Transaction)?,
    })
  }
}

#[derive(Debug, PartialEq)]
pub enum Error {
  Separator,
  Block(ParseIntError),
  Transaction(ParseIntError),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Separator => write!(f, "missing separator"),
      Self::Block(err) => write!(f, "invalid height: {err}"),
      Self::Transaction(err) => write!(f, "invalid index: {err}"),
    }
  }
}

impl std::error::Error for Error {}

