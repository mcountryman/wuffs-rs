use crate::types::IntoWuffsSlice;

pub mod adler32;
pub mod crc32;

pub trait WuffsHash {
  fn update(&mut self, buf: impl IntoWuffsSlice) -> u32;
}
