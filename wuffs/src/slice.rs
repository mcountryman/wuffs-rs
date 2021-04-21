use std::marker::PhantomData;

use wuffs_sys::wuffs_base__slice_u8;

pub struct WuffsSlice<'a, T: WuffsSliceElement> {
  inner: T::Native,
  phantom: PhantomData<&'a T>,
}

impl<T: WuffsSliceElement> WuffsSlice<'_, T> {
  pub fn len(&self) -> usize {
    T::len(&self.inner)
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn data(&self) -> &[T] {
    T::data(&self.inner)
  }

  pub fn into_inner(self) -> T::Native {
    self.inner
  }
}

impl<T: WuffsSliceElement> AsRef<[T]> for WuffsSlice<'_, T> {
  fn as_ref(&self) -> &[T] {
    self.data()
  }
}

pub trait WuffsSliceElement: Sized {
  type Native;

  fn len(imp: &Self::Native) -> usize;
  fn data(imp: &Self::Native) -> &[Self];
  fn from(slice: &[Self]) -> Self::Native;
}

impl<T: WuffsSliceElement> From<&[T]> for WuffsSlice<'_, T> {
  fn from(slice: &[T]) -> Self {
    WuffsSlice {
      inner: T::from(slice),
      phantom: Default::default(),
    }
  }
}

impl<T: WuffsSliceElement, const S: usize> From<&[T; S]> for WuffsSlice<'_, T> {
  fn from(slice: &[T; S]) -> Self {
    WuffsSlice {
      inner: T::from(slice),
      phantom: Default::default(),
    }
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

  fn from(slice: &[Self]) -> Self::Native {
    wuffs_base__slice_u8 {
      ptr: slice.as_ptr() as *mut _,
      len: slice.len() as _,
    }
  }
}
