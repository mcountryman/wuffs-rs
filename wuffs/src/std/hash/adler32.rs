use crate::{
  status::{IntoResult, WuffsError},
  types::{IntoWuffsSlice, WuffsBox},
};
use wuffs_sys::*;

use super::WuffsHash;

#[derive(Clone)]
pub struct WuffsAdler32(WuffsBox<wuffs_adler32__hasher>);

impl WuffsAdler32 {
  pub fn new() -> Result<Self, WuffsError> {
    unsafe {
      let size = sizeof__wuffs_adler32__hasher();
      let mut inner = WuffsBox::new(size as _);

      wuffs_adler32__hasher__initialize(
        //
        inner.as_mut_ptr(),
        size,
        WUFFS_VERSION as _,
        0x00000001, // WUFFS_INITIALIZE__ALREADY_ZEROED
      )
      .into_result()?;

      Ok(Self(inner))
    }
  }
}

impl WuffsHash for WuffsAdler32 {
  fn update(&mut self, buf: impl IntoWuffsSlice) -> u32 {
    unsafe {
      wuffs_adler32__hasher__update_u32(self.0.as_mut_ptr(), buf.into_wuffs_slice_u8())
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::std::hash::WuffsHash;

  #[test]
  fn test_adler32() {
    let mut adler = super::WuffsAdler32::new().unwrap();
    let sum = adler.update(b"rust is pretty cool, man");

    assert_eq!(sum, 1921255656);
  }
}
