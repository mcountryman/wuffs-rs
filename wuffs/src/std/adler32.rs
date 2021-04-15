use wuffs_sys::{
  sizeof__wuffs_adler32__hasher, wuffs_adler32__hasher,
  wuffs_adler32__hasher__initialize, wuffs_adler32__hasher__update_u32,
  wuffs_base__slice_u8, WUFFS_VERSION,
};

use crate::status::{IntoResult, WuffsError};

pub struct WuffsAdler32(*mut wuffs_adler32__hasher);

impl WuffsAdler32 {
  pub fn new() -> Result<Self, WuffsError> {
    unsafe {
      let size = sizeof__wuffs_adler32__hasher();
      let inner = libc::calloc(size as _, 1);
      let inner = inner as *mut _;

      wuffs_adler32__hasher__initialize(
        //
        inner,
        size,
        WUFFS_VERSION as _,
        0x00000001, // WUFFS_INITIALIZE__ALREADY_ZEROED
      )
      .into_result()?;

      Ok(Self(inner))
    }
  }

  pub fn update(&mut self, buf: &[u8]) -> u32 {
    unsafe {
      let mut slice = buf.to_vec();
      let slice_ptr = slice.as_mut_ptr();
      #[allow(clippy::forget_copy)]
      std::mem::forget(slice);

      let slice = wuffs_base__slice_u8 {
        ptr: slice_ptr,
        len: buf.len() as _,
      };

      wuffs_adler32__hasher__update_u32(self.0, slice)
    }
  }
}

impl Drop for WuffsAdler32 {
  fn drop(&mut self) {
    unsafe { libc::free(self.0 as *mut _) }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_adler32() {
    let mut adler = super::WuffsAdler32::new().unwrap();
    let sum = adler.update(b"rust is pretty cool, man");

    assert_eq!(sum, 1921255656);
  }
}
