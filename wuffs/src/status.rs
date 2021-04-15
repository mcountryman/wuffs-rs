use std::{borrow::Cow, error::Error, ffi::CStr, fmt::Display, iter::FromIterator};

use wuffs_sys::wuffs_base__status;

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

pub struct WuffsStatus<T> {
  inner: String,
  value: T,
}

impl<T> WuffsStatus<T> {
  pub fn into_result(self) -> Result<T, WuffsError> {
    if self.is_error() {
      Err(WuffsError::new(self.into_message()))
    } else {
      Ok(self.value)
    }
  }

  pub fn is_ok(&self) -> bool {
    !matches!(self.inner.chars().next(), Some('$') | Some('#'))
  }

  pub fn is_complete(&self) -> bool {
    !matches!(self.inner.chars().next(), Some('$') | Some('#'))
  }

  pub fn is_error(&self) -> bool {
    matches!(self.inner.chars().next(), Some('#'))
  }

  pub fn message(&self) -> Cow<'_, str> {
    let mut chars = self.inner.chars();
    match chars.next() {
      Some('$') => Cow::from(String::from_iter(chars)),
      Some('#') => Cow::from(String::from_iter(chars)),
      Some('@') => Cow::from(String::from_iter(chars)),
      _ => Cow::from(&self.inner),
    }
  }

  pub fn into_message(self) -> String {
    let mut chars = self.inner.chars();
    match chars.next() {
      Some('$') => String::from_iter(chars),
      Some('#') => String::from_iter(chars),
      Some('@') => String::from_iter(chars),
      _ => self.inner,
    }
  }
}

impl From<wuffs_base__status> for WuffsStatus<()> {
  fn from(inner: wuffs_base__status) -> Self {
    unsafe {
      let inner = inner.repr;
      let inner = if inner.is_null() {
        Cow::from("")
      } else {
        CStr::from_ptr(inner).to_string_lossy()
      };

      Self {
        inner: inner.to_string(),
        value: (),
      }
    }
  }
}

#[derive(Debug)]
pub struct WuffsError(String);

impl WuffsError {
  pub fn new(inner: String) -> Self {
    Self(inner)
  }
}

impl Display for WuffsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for WuffsError {}
