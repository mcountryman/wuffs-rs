use std::marker::PhantomData;
use wuffs_sys::wuffs_base__slice_u8;

/// Heap allocated type with runtime defined size.
#[derive(Clone)]
pub struct WuffsBox<T> {
  handle: Vec<u8>,
  phantom: PhantomData<T>,
}

impl<T> WuffsBox<T> {
  /// Initialize WuffsBox with supplied size.
  pub fn new(size: usize) -> Self {
    Self {
      handle: vec![0; size],
      phantom: Default::default(),
    }
  }

  /// Cast to pointer
  ///
  /// # Safety
  /// Conversion assumes that underlying allocated buffer is correct size.  Ideally this
  /// will use the `sizeof__wuffs_*` methods.
  pub unsafe fn as_ptr(&self) -> *const T {
    self.handle.as_ptr() as *const T
  }

  /// Cast to mutable pointer
  ///
  /// # Safety
  /// Conversion assumes that underlying allocated buffer is correct size.  Ideally this
  /// will use the `sizeof__wuffs_*` methods.
  pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
    self.handle.as_mut_ptr() as *mut T
  }
}

/// Convert type into `wuffs_base__slice_u8`
pub trait IntoWuffsSlice {
  fn into_wuffs_slice_u8(self) -> wuffs_base__slice_u8;
}

impl<A: AsRef<[u8]>> IntoWuffsSlice for A {
  fn into_wuffs_slice_u8(self) -> wuffs_base__slice_u8 {
    let buf = self.as_ref();

    wuffs_base__slice_u8 {
      ptr: buf.as_ptr() as *mut _,
      len: buf.len() as _,
    }
  }
}
