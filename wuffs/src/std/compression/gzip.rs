use std::{io::BufRead, ptr::null_mut};

use crate::{
  boxed::{WuffsBox, WuffsBoxed},
  slice::WuffsSliceOwned,
  status::{IntoResult, WuffsError, WuffsStatus},
};
use wuffs_sys::{
  sizeof__wuffs_gzip__decoder, wuffs_gzip__decoder, wuffs_gzip__decoder__initialize,
  wuffs_gzip__decoder__set_quirk_enabled, wuffs_gzip__decoder__transform_io,
  WUFFS_GZIP__DECODER_WORKBUF_LEN_MAX_INCL_WORST_CASE as WORK_BUF_SIZE, WUFFS_VERSION,
};

#[derive(Clone)]
pub struct WuffsGzipDecoder(WuffsBox<wuffs_gzip__decoder>);

impl WuffsGzipDecoder {
  pub fn new() -> Result<Self, WuffsError> {
    let mut inner = WuffsBox::new();

    unsafe {
      wuffs_gzip__decoder__initialize(
        inner.as_mut_ptr(),
        inner.size() as _,
        WUFFS_VERSION as _,
        0x01,
      )
      .into_result()?;
    }

    Ok(Self(inner))
  }

  pub fn set_quirk_enabled(&mut self, quirk: u32, enabled: bool) {
    unsafe {
      wuffs_gzip__decoder__set_quirk_enabled(self.0.as_mut_ptr(), quirk, enabled);
    }
  }

  pub fn decode<R, W>(&mut self, reader: &mut R, writer: &mut W) {
    let work_buf = vec![0; WORK_BUF_SIZE as _];
    let work_buf = WuffsSliceOwned::new(work_buf);

    let status: WuffsStatus<()> = unsafe {
      wuffs_gzip__decoder__transform_io(
        self.0.as_mut_ptr(),
        null_mut(),
        null_mut(),
        work_buf.into_inner(),
      )
      .into()
    };
    //
    // wuffs_base__io_transformer* self,
    // wuffs_base__io_buffer* a_dst,
    // wuffs_base__io_buffer* a_src,
    // wuffs_base__slice_u8 a_workbuf
    //
  }
}

impl WuffsBoxed for wuffs_gzip__decoder {
  fn size() -> usize {
    unsafe { sizeof__wuffs_gzip__decoder() as _ }
  }
}
