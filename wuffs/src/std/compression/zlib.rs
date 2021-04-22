use crate::{
  boxed::{WuffsBox, WuffsBoxed},
  buf::WuffsBuf,
  slice::WuffsSlice,
  status::{IntoResult, WuffsError},
};
use wuffs_sys::{
  sizeof__wuffs_zlib__decoder, wuffs_zlib__decoder, wuffs_zlib__decoder__initialize,
  wuffs_zlib__decoder__set_quirk_enabled, wuffs_zlib__decoder__transform_io,
  WUFFS_VERSION, WUFFS_ZLIB__DECODER_WORKBUF_LEN_MAX_INCL_WORST_CASE as WORK_BUF_SIZE,
};

#[derive(Clone)]
pub struct WuffsZlibDecoder {
  work: Vec<u8>,
  inner: WuffsBox<wuffs_zlib__decoder>,
}

impl WuffsZlibDecoder {
  pub fn new() -> Result<Self, WuffsError> {
    let mut inner = WuffsBox::new();

    unsafe {
      wuffs_zlib__decoder__initialize(
        inner.as_mut_ptr(),
        inner.size() as _,
        WUFFS_VERSION as _,
        0x01,
      )
      .into_result()?;
    }

    Ok(Self {
      work: vec![0; WORK_BUF_SIZE as _],
      inner,
    })
  }

  pub fn set_quirk_enabled(&mut self, quirk: u32, enabled: bool) {
    unsafe {
      wuffs_zlib__decoder__set_quirk_enabled(self.inner.as_mut_ptr(), quirk, enabled);
    }
  }

  pub fn decode(
    &mut self,
    src: &[u8],
    dst: &mut [u8],
  ) -> Result<(usize, usize), WuffsError> {
    let mut src = unsafe { WuffsBuf::from_slice_readonly(src) };
    let mut dst = WuffsBuf::from_slice(dst);

    let work = WuffsSlice::from(&mut self.work[..]);

    unsafe {
      wuffs_zlib__decoder__transform_io(
        self.inner.as_mut_ptr(),
        dst.as_mut_ptr(),
        src.as_mut_ptr(),
        work.into_inner(),
      )
      .into_result()?
    };

    Ok((src.read(), dst.written()))
  }
}

impl WuffsBoxed for wuffs_zlib__decoder {
  fn size() -> usize {
    unsafe { sizeof__wuffs_zlib__decoder() as _ }
  }
}
