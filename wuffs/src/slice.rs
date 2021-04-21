use wuffs_sys::wuffs_base__slice_u8;

pub struct WuffsSlice<T: WuffsSliceElement> {
  data: Vec<T>,
  inner: T::Native,
}

impl<T: WuffsSliceElement> WuffsSlice<T> {
  pub fn new(mut data: Vec<T>) -> Self {
    let inner = T::from(&mut data);

    Self { data, inner }
  }

  pub fn len(&self) -> usize {
    T::len(&self.inner)
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn data(&self) -> &[T] {
    &self.data
  }

  pub fn into_inner(self) -> T::Native {
    self.inner
  }

  /// Convert read-only slice reference into a `T::Native`
  ///
  /// # Safety
  /// Potential cast to `*mut` ptr underlying with the hope the underlying data isn't
  /// modified.
  pub unsafe fn into_readonly(slice: &[T]) -> T::Native {
    T::from_readonly(slice)
  }
}

impl<T: WuffsSliceElement> AsRef<[T]> for WuffsSlice<T> {
  fn as_ref(&self) -> &[T] {
    self.data()
  }
}

pub trait WuffsSliceElement: Sized + Clone {
  type Native;

  fn len(imp: &Self::Native) -> usize;
  fn data(imp: &Self::Native) -> &[Self];
  fn from(slice: &mut [Self]) -> Self::Native;

  /// Convert read-only slice reference into a `Self::Native`
  ///
  /// # Safety
  /// Potential cast to `*mut` ptr underlying with the hope the underlying data isn't
  /// modified.
  unsafe fn from_readonly(slice: &[Self]) -> Self::Native;
}

impl<T: WuffsSliceElement> From<&[T]> for WuffsSlice<T> {
  fn from(slice: &[T]) -> Self {
    WuffsSlice::new(slice.to_vec())
  }
}

impl<T: WuffsSliceElement, const S: usize> From<&[T; S]> for WuffsSlice<T> {
  fn from(slice: &[T; S]) -> Self {
    WuffsSlice::new(slice.to_vec())
  }
}

impl WuffsSliceElement for u8 {
  type Native = wuffs_base__slice_u8;

  fn len(imp: &Self::Native) -> usize {
    imp.len as _
  }

  fn data(imp: &Self::Native) -> &[Self] {
    unsafe { std::slice::from_raw_parts(imp.ptr, imp.len as _) }
  }

  fn from(slice: &mut [Self]) -> Self::Native {
    wuffs_base__slice_u8 {
      ptr: slice.as_mut_ptr() as *mut _,
      len: slice.len() as _,
    }
  }

  unsafe fn from_readonly(slice: &[Self]) -> Self::Native {
    wuffs_base__slice_u8 {
      ptr: slice.as_ptr() as *mut _,
      len: slice.len() as _,
    }
  }
}
