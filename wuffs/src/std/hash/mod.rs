pub mod adler32;
pub mod crc32;

pub trait WuffsHash {
  fn update<S>(&mut self, buf: S) -> u32
  where
    S: AsRef<[u8]>;
}
