use std::{marker::PhantomData, ops::Deref};

use wuffs_sys::wuffs_base__slice_u8;

#[derive(Clone)]
pub enum WuffsSlice<'a, T: WuffsSliceImpl> {
  Owned(WuffsSliceOwned<T>),
  Borrowed(WuffsSliceBorrowed<'a, T>),
}

impl<T: WuffsSliceImpl> WuffsSlice<'_, T> {
  pub fn from(inner: T::Native) -> Self {
    Self::Borrowed(WuffsSliceBorrowed::from(inner))
  }

  /// Convert read-only slice reference into a `T::Native`
  ///
  /// # Safety
  /// Potential cast to `*mut` ptr underlying with the hope the underlying data isn't
  /// modified.
  pub unsafe fn from_readonly(slice: &[T]) -> T::Native {
    T::from_readonly(slice)
  }
}

impl<T: WuffsSliceImpl> AsRef<[T]> for WuffsSlice<'_, T> {
  fn as_ref(&self) -> &[T] {
    self.data()
  }
}

impl<'a, T: WuffsSliceImpl> From<&'a mut [T]> for WuffsSlice<'a, T> {
  fn from(slice: &'a mut [T]) -> Self {
    Self::Borrowed(WuffsSliceBorrowed::new(slice))
  }
}

impl<'a, T: WuffsSliceImpl> From<Vec<T>> for WuffsSlice<'a, T> {
  fn from(vec: Vec<T>) -> Self {
    Self::Owned(WuffsSliceOwned::new(vec))
  }
}

impl<T: WuffsSliceImpl> Deref for WuffsSlice<'_, T> {
  type Target = T::Native;

  fn deref(&self) -> &Self::Target {
    match self {
      WuffsSlice::Owned(owned) => owned,
      WuffsSlice::Borrowed(borrowed) => borrowed,
    }
  }
}

#[derive(Clone)]
pub struct WuffsSliceOwned<T: WuffsSliceImpl> {
  data: Vec<T>,
  inner: T::Native,
}

impl<T: WuffsSliceImpl> WuffsSliceOwned<T> {
  pub fn new(mut data: Vec<T>) -> Self {
    let inner = T::from(&mut data);

    Self { data, inner }
  }

  pub fn into_data(self) -> Vec<T> {
    self.data
  }
}

impl<T: WuffsSliceImpl> Deref for WuffsSliceOwned<T> {
  type Target = T::Native;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[derive(Clone)]
pub struct WuffsSliceBorrowed<'a, T: WuffsSliceImpl> {
  inner: T::Native,
  phantom: PhantomData<&'a u8>,
}

impl<'a, T: WuffsSliceImpl> WuffsSliceBorrowed<'a, T> {
  pub fn new(slice: &'a mut [T]) -> Self {
    Self {
      inner: T::from(slice),
      phantom: Default::default(),
    }
  }

  pub fn from(inner: T::Native) -> Self {
    Self {
      inner,
      phantom: Default::default(),
    }
  }
}

impl<T: WuffsSliceImpl> Deref for WuffsSliceBorrowed<'_, T> {
  type Target = T::Native;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

pub trait WuffsSliceImpl: Sized + Clone {
  type Native: WuffsSliceNative<Self> + Clone;

  fn from(slice: &mut [Self]) -> Self::Native;

  /// Convert read-only slice reference into a `Self::Native`
  ///
  /// # Safety
  /// Potential cast to `*mut` ptr underlying with the hope the underlying data isn't
  /// modified.
  unsafe fn from_readonly(slice: &[Self]) -> Self::Native;
}

pub trait WuffsSliceNative<T> {
  fn data(&self) -> &[T];
}

impl WuffsSliceImpl for u8 {
  type Native = wuffs_base__slice_u8;

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

impl WuffsSliceNative<u8> for wuffs_base__slice_u8 {
  fn data(&self) -> &[u8] {
    unsafe { std::slice::from_raw_parts(self.ptr, self.len as _) }
  }
}
