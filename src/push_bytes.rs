use super::*;


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushBytes([u8]);



impl PushBytes {
    /// Returns the number of bytes in buffer.
    pub fn len(&self) -> usize { self.as_bytes().len() }

    /// Returns true if the buffer contains zero bytes.
    pub fn is_empty(&self) -> bool { self.as_bytes().is_empty() }
    /// Creates `&Self` without checking the length.
    ///
    /// ## Safety
    ///
    /// The caller is responsible for checking that the length is less than the [`LIMIT`].
    unsafe fn from_slice_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes as *const [u8] as *const PushBytes)
    }

    /// Creates `&mut Self` without checking the length.
    ///
    /// ## Safety
    ///
    /// The caller is responsible for checking that the length is less than the [`LIMIT`].
    unsafe fn from_mut_slice_unchecked(bytes: &mut [u8]) -> &mut Self {
        &mut *(bytes as *mut [u8] as *mut PushBytes)
    }

    /// Creates an empty `PushBytes`.
    pub fn empty() -> &'static Self {
        // 0 < LIMIT
        unsafe { Self::from_slice_unchecked(&[]) }
    }

    /// Returns the underlying bytes.
    pub fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Returns the underlying mutbale bytes.
    pub fn as_mut_bytes(&mut self) -> &mut [u8] { &mut self.0 }
}


// fn check_limit(len: usize) -> Result<(), PushBytesError> {
//   if len < 0x100000000 {
//       Ok(())
//   } else {
//       Err(PushBytesError { len })
//   }
// }


#[derive(Debug)]
pub struct PushBytesError {
  /// How long the input was.
  pub(super) len: usize,
}


impl<'a> TryFrom<&'a [u8]> for &'a PushBytes {
  type Error = PushBytesError;

  fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
    // check_limit(bytes.len())?;
    // We've just checked the length
    Ok(unsafe { PushBytes::from_slice_unchecked(bytes) })
  }
}