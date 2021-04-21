use crate::slice::WuffsSlice;
use wuffs_sys::{wuffs_base__io_buffer, wuffs_base__io_buffer_meta};

#[derive(Debug, Clone, Copy)]
pub struct WuffsBuf(wuffs_base__io_buffer);

impl WuffsBuf {
  pub fn from_slice(buf: &mut [u8]) -> Self {
    Self(wuffs_base__io_buffer {
      data: WuffsSlice::from(buf).into_inner(),
      meta: wuffs_base__io_buffer_meta {
        wi: 0,
        ri: 0,
        pos: 0,
        closed: false,
      },
    })
  }

  pub unsafe fn from_slice_readonly(buf: &[u8]) -> Self {
    let len = buf.len() as _;

    Self(wuffs_base__io_buffer {
      data: WuffsSlice::from_readonly(buf),
      meta: wuffs_base__io_buffer_meta {
        wi: len,
        ri: 0,
        pos: 0,
        closed: true,
      },
    })
  }

  pub fn as_mut_ptr(&mut self) -> *mut wuffs_base__io_buffer {
    &mut self.0 as *mut _
  }

  pub fn len(&self) -> usize {
    self.0.data.len as _
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn pos(&self) -> usize {
    self.0.meta.pos as _
  }

  pub fn read(&self) -> usize {
    self.0.meta.ri as _
  }

  pub fn written(&self) -> usize {
    self.0.meta.wi as _
  }
}
