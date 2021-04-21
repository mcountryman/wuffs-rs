use crate::slice::WuffsSlice;

pub mod adler32;
pub mod crc32;

pub trait WuffsHash {
  fn update<'a, S>(&mut self, buf: S) -> u32
  where
    S: Into<WuffsSlice<'a, u8>>;
}
