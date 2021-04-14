use wuffs_sys::{
  sizeof__wuffs_adler32__hasher, wuffs_adler32__hasher, wuffs_adler32__hasher__alloc,
  wuffs_adler32__hasher__initialize, wuffs_adler32__hasher__update_u32,
  wuffs_base__slice_u8, WUFFS_VERSION,
};

pub struct WuffsAdler32 {
  inner: wuffs_adler32__hasher,
}

impl WuffsAdler32 {
  pub fn new() -> Self {
    unsafe {
      let inner = wuffs_adler32__hasher__alloc();

      wuffs_adler32__hasher__initialize(
        inner,
        sizeof__wuffs_adler32__hasher(),
        WUFFS_VERSION as _,
        0,
      );

      // TODO: Check status

      Self { inner: *inner }
    }
  }

  pub fn update(&mut self, buf: &[u8]) -> u32 {
    unsafe {
      let mut slice = buf.to_vec();
      let slice = wuffs_base__slice_u8 {
        ptr: slice.as_mut_ptr(),
        len: buf.len() as _,
      };

      wuffs_adler32__hasher__update_u32(&mut self.inner as *mut _, slice)
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_adler32() {
    let mut adler = super::WuffsAdler32::new();
    let sum = adler.update(b"");
  }
}
