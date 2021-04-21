use std::{borrow::Cow, error::Error, ffi::CStr, fmt::Display, iter::FromIterator};

use wuffs_sys::{
  wuffs_base__status, wuffs_base__suspension__even_more_information,
  wuffs_base__suspension__mispositioned_read,
  wuffs_base__suspension__mispositioned_write, wuffs_base__suspension__short_read,
  wuffs_base__suspension__short_write,
};

pub trait IntoResult<T> {
  fn into_result(self) -> Result<T, WuffsError>;
}

impl<S, T> IntoResult<T> for S
where
  S: Into<WuffsStatus<T>>,
{
  fn into_result(self) -> Result<T, WuffsError> {
    let status: WuffsStatus<T> = self.into();
    status.into_result()
  }
}

#[derive(Debug, Clone)]
pub enum WuffsStatus<T = ()> {
  Ok(T),
  Err(WuffsError),
  Suspension(WuffsSuspension),
}

#[derive(Debug, Clone)]
pub enum WuffsError {
  Message(String),
  Suspension(WuffsSuspension),
}

#[derive(Debug, Copy, Clone)]
pub enum WuffsSuspension {
  Unknown,
  EvenMoreInformation,
  MispositionedRead,
  MispositionedWrite,
  ShortRead,
  ShortWrite,
}

impl<T> WuffsStatus<T> {
  pub fn into_result(self) -> Result<T, WuffsError> {
    match self {
      Self::Ok(value) => Ok(value),
      Self::Err(err) => Err(err),
      Self::Suspension(suspension) => Err(WuffsError::Suspension(suspension)),
    }
  }
}

impl From<wuffs_base__status> for WuffsStatus<()> {
  fn from(inner: wuffs_base__status) -> Self {
    let repr = unsafe {
      if inner.repr.is_null() {
        Cow::from("")
      } else {
        CStr::from_ptr(inner.repr).to_string_lossy()
      }
    };

    let mut chars = repr.chars();
    match chars.next() {
      Some('$') => WuffsStatus::Suspension(WuffsSuspension::from_ptr(inner.repr)),
      Some('#') => WuffsStatus::Err(chars.collect()),
      // Some('@') => WuffsStatus::Note(?),
      _ => WuffsStatus::Ok(()),
    }
  }
}

impl WuffsSuspension {
  pub fn from_ptr(ptr: *const i8) -> Self {
    unsafe {
      if ptr == wuffs_base__suspension__even_more_information.as_ptr() {
        WuffsSuspension::EvenMoreInformation
      } else if ptr == wuffs_base__suspension__mispositioned_read.as_ptr() {
        WuffsSuspension::MispositionedRead
      } else if ptr == wuffs_base__suspension__mispositioned_write.as_ptr() {
        WuffsSuspension::MispositionedWrite
      } else if ptr == wuffs_base__suspension__short_read.as_ptr() {
        WuffsSuspension::ShortRead
      } else if ptr == wuffs_base__suspension__short_write.as_ptr() {
        WuffsSuspension::ShortWrite
      } else {
        WuffsSuspension::Unknown
      }
    }
  }
}

impl FromIterator<char> for WuffsError {
  fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
    Self::Message(String::from_iter(iter))
  }
}

impl Display for WuffsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for WuffsError {}
