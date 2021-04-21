use super::WuffsHash;
use crate::{
  boxed::{WuffsBox, WuffsBoxed},
  slice::WuffsSlice,
  status::{IntoResult, WuffsError},
};
use wuffs_sys::*;

#[derive(Clone)]
pub struct WuffsCrc32(WuffsBox<wuffs_crc32__ieee_hasher>);

impl WuffsCrc32 {
  pub fn new() -> Result<Self, WuffsError> {
    unsafe {
      let mut inner = WuffsBox::new();

      wuffs_crc32__ieee_hasher__initialize(
        //
        inner.as_mut_ptr(),
        inner.size() as _,
        WUFFS_VERSION as _,
        0x00000001, // WUFFS_INITIALIZE__ALREADY_ZEROED
      )
      .into_result()?;

      Ok(Self(inner))
    }
  }
}

impl WuffsHash for WuffsCrc32 {
  fn update<'a, S>(&mut self, buf: S) -> u32
  where
    S: Into<WuffsSlice<'a, u8>>,
  {
    unsafe {
      wuffs_crc32__ieee_hasher__update_u32(self.0.as_mut_ptr(), buf.into().into_inner())
    }
  }
}

impl WuffsBoxed for wuffs_crc32__ieee_hasher {
  fn size() -> usize {
    unsafe { sizeof__wuffs_crc32__ieee_hasher() as _ }
  }
}

#[cfg(test)]
mod tests {
  use crate::std::hash::WuffsHash;

  #[test]
  fn test_crc32() {
    let mut adler = super::WuffsCrc32::new().unwrap();
    let sum = adler.update(b"rust is pretty cool, man");

    assert_eq!(sum, 0xb80184ca);
  }
}
