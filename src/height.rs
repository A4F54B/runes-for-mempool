use std::{fmt::Display};

use super::*;

pub const DIFFCHANGE_INTERVAL: u32 = 2016;

#[derive(Copy, Clone, Debug, Display, FromStr, Ord, Eq, Serialize, PartialEq, PartialOrd)]
pub struct Height(pub u32);

impl Height {
  pub fn n(self) -> u32 {
    self.0
  }

  pub fn subsidy(self) -> u64 {
    Epoch::from(self).subsidy()
  }

  pub fn starting_sat(self) -> Sat {
    let epoch = Epoch::from(self);
    let epoch_starting_sat = epoch.starting_sat();
    let epoch_starting_height = epoch.starting_height();
    epoch_starting_sat + u64::from(self.n() - epoch_starting_height.n()) * epoch.subsidy()
  }

  pub fn period_offset(self) -> u32 {
    self.0 % DIFFCHANGE_INTERVAL
  }
}

impl Add<u32> for Height {
  type Output = Self;

  fn add(self, other: u32) -> Height {
    Self(self.0 + other)
  }
}

impl Sub<u32> for Height {
  type Output = Self;

  fn sub(self, other: u32) -> Height {
    Self(self.0 - other)
  }
}

impl PartialEq<u32> for Height {
  fn eq(&self, other: &u32) -> bool {
    self.0 == *other
  }
}

