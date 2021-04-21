use std::marker::PhantomData;

/// Heap allocated type with runtime defined size.
#[derive(Clone)]
pub struct WuffsBox<T: WuffsBoxed> {
  handle: Vec<u8>,
  phantom: PhantomData<T>,
}

impl<T: WuffsBoxed> WuffsBox<T> {
  /// Initialize WuffsBox with supplied size.
  pub fn new() -> Self {
    Self {
      handle: vec![0; T::size()],
      phantom: Default::default(),
    }
  }

  pub fn size(&self) -> usize {
    T::size()
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

impl<T: WuffsBoxed> Default for WuffsBox<T> {
  fn default() -> Self {
    Self::new()
  }
}

pub trait WuffsBoxed {
  fn size() -> usize;
}
