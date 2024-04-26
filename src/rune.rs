use super::*;

enum Network {
  /// Mainnet Bitcoin.
  Bitcoin,
  /// Bitcoin's testnet network.
  Testnet,
  /// Bitcoin's signet network.
  Signet,
  /// Bitcoin's regtest network.
  Regtest,
}

const SUBSIDY_HALVING_INTERVAL: u32 = 210_000;


#[derive(
  Default, Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq, DeserializeFromStr, SerializeDisplay,
)]
pub struct Rune(pub u128);

impl Rune {
  const RESERVED: u128 = 6402364363415443603228541259936211926;

  const STEPS: &'static [u128] = &[
    0,
    26,
    702,
    18278,
    475254,
    12356630,
    321272406,
    8353082582,
    217180147158,
    5646683826134,
    146813779479510,
    3817158266467286,
    99246114928149462,
    2580398988131886038,
    67090373691429037014,
    1744349715977154962390,
    45353092615406029022166,
    1179180408000556754576342,
    30658690608014475618984918,
    797125955808376366093607894,
    20725274851017785518433805270,
    538857146126462423479278937046,
    14010285799288023010461252363222,
    364267430781488598271992561443798,
    9470953200318703555071806597538774,
    246244783208286292431866971536008150,
    6402364363415443603228541259936211926,
    166461473448801533683942072758341510102,
  ];

  pub fn n(self) -> u128 {
    self.0
  }

  // pub fn first_rune_height(network: Network) -> u32 {
  //   SUBSIDY_HALVING_INTERVAL
  //     * match network {
  //       Network::Bitcoin => 4,
  //       Network::Regtest => 0,
  //       Network::Signet => 0,
  //       Network::Testnet => 12,
  //       _ => 0,
  //     }
  // }

  // pub fn minimum_at_height(chain: Network, height: Height) -> Self {
  //   let offset = height.0.saturating_add(1);

  //   const INTERVAL: u32 = SUBSIDY_HALVING_INTERVAL / 12;

  //   let start = Self::first_rune_height(chain);

  //   let end = start + SUBSIDY_HALVING_INTERVAL;

  //   if offset < start {
  //     return Rune(Self::STEPS[12]);
  //   }

  //   if offset >= end {
  //     return Rune(0);
  //   }

  //   let progress = offset.saturating_sub(start);

  //   let length = 12u32.saturating_sub(progress / INTERVAL);

  //   let end = Self::STEPS[usize::try_from(length - 1).unwrap()];

  //   let start = Self::STEPS[usize::try_from(length).unwrap()];

  //   let remainder = u128::from(progress % INTERVAL);

  //   Rune(start - ((start - end) * remainder / u128::from(INTERVAL)))
  // }

  pub fn is_reserved(self) -> bool {
    self.0 >= Self::RESERVED
  }

  pub fn reserved(block: u64, tx: u32) -> Self {
    Self(
      Self::RESERVED
        .checked_add(u128::from(block) << 32 | u128::from(tx))
        .unwrap(),
    )
  }

  pub fn commitment(self) -> Vec<u8> {
    let bytes = self.0.to_le_bytes();

    let mut end = bytes.len();

    while end > 0 && bytes[end - 1] == 0 {
      end -= 1;
    }

    bytes[..end].into()
  }
}

impl Display for Rune {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let mut n = self.0;
    if n == u128::MAX {
      return write!(f, "BCGDENLQRQWDSLRUGSNLBTMFIJAV");
    }

    n += 1;
    let mut symbol = String::new();
    while n > 0 {
      symbol.push(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
          .chars()
          .nth(((n - 1) % 26) as usize)
          .unwrap(),
      );
      n = (n - 1) / 26;
    }

    for c in symbol.chars().rev() {
      write!(f, "{c}")?;
    }

    Ok(())
  }
}

impl FromStr for Rune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Error> {
    let mut x = 0u128;
    for (i, c) in s.chars().enumerate() {
      if i > 0 {
        x = x.checked_add(1).ok_or(Error::Range)?;
      }
      x = x.checked_mul(26).ok_or(Error::Range)?;
      match c {
        'A'..='Z' => {
          x = x.checked_add(c as u128 - 'A' as u128).ok_or(Error::Range)?;
        }
        _ => return Err(Error::Character(c)),
      }
    }
    Ok(Rune(x))
  }
}

#[derive(Debug, PartialEq)]
pub enum Error {
  Character(char),
  Range,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Character(c) => write!(f, "invalid character `{c}`"),
      Self::Range => write!(f, "name out of range"),
    }
  }
}

impl std::error::Error for Error {}

