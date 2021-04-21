use crate::slice::WuffsSlice;
use wuffs_sys::{wuffs_base__io_buffer, wuffs_base__io_buffer_meta};

#[derive(Copy, Clone)]
pub struct WuffsBuf(wuffs_base__io_buffer);

#[derive(Copy, Clone)]
pub struct WuffsBufferMeta(wuffs_base__io_buffer_meta);

impl WuffsBuf {
  pub fn with_capacity(_: usize) -> WuffsBuf {
    todo!()
  }

  pub fn data(&self) -> WuffsSlice<'_, u8> {
    WuffsSlice::from(self.0.data)
  }

  pub fn meta(&self) -> WuffsBufferMeta {
    WuffsBufferMeta(self.0.meta)
  }
}

impl WuffsBufferMeta {
  pub fn pos(&self) -> usize {
    self.0.pos as _
  }

  pub fn closed(&self) -> bool {
    self.0.closed
  }

  pub fn write_increment(&self) -> usize {
    self.0.wi as _
  }

  pub fn read_increment(&self) -> usize {
    self.0.ri as _
  }

  pub fn into_inner(self) -> wuffs_base__io_buffer_meta {
    self.0
  }
}
