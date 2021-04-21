use crate::{
  boxed::{WuffsBox, WuffsBoxed},
  buf::WuffsBuf,
  slice::WuffsSlice,
  status::{IntoResult, WuffsError},
};
use wuffs_sys::{
  sizeof__wuffs_gzip__decoder, wuffs_gzip__decoder, wuffs_gzip__decoder__initialize,
  wuffs_gzip__decoder__set_quirk_enabled, wuffs_gzip__decoder__transform_io,
  WUFFS_GZIP__DECODER_WORKBUF_LEN_MAX_INCL_WORST_CASE as WORK_BUF_SIZE, WUFFS_VERSION,
};

#[derive(Clone)]
pub struct WuffsGzipDecoder {
  work: Vec<u8>,
  inner: WuffsBox<wuffs_gzip__decoder>,
}

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

    Ok(Self {
      work: vec![0; WORK_BUF_SIZE as _],
      inner,
    })
  }

  pub fn set_quirk_enabled(&mut self, quirk: u32, enabled: bool) {
    unsafe {
      wuffs_gzip__decoder__set_quirk_enabled(self.inner.as_mut_ptr(), quirk, enabled);
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
      wuffs_gzip__decoder__transform_io(
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

impl WuffsBoxed for wuffs_gzip__decoder {
  fn size() -> usize {
    unsafe { sizeof__wuffs_gzip__decoder() as _ }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_gzip_decode() {
    let cmp = b"Hello Wuffs.\n";

    // src holds a gzip-encoded "Hello Wuffs."
    //
    // $ echo "Hello Wuffs." | gzip --no-name | xxd
    // 00000000: 1f8b 0800 0000 0000 0003 f348 cdc9 c957  ...........H...W
    // 00000010: 082f 4d4b 2bd6 e302 003c 8475 bb0d 0000  ./MK+....<.u....
    // 00000020: 00                                       .
    //
    // Passing --no-name to the gzip command line also means to skip the timestamp,
    // which means that its output is deterministic.
    let src = [
      0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // 00..07
      0x00, 0x03, 0xf3, 0x48, 0xcd, 0xc9, 0xc9, 0x57, // 08..0F
      0x08, 0x2f, 0x4d, 0x4b, 0x2b, 0xd6, 0xe3, 0x02, // 10..17
      0x00, 0x3c, 0x84, 0x75, 0xbb, 0x0d, 0x00, 0x00, // 18..1F
      0x00, // 20..20
    ];

    let mut dst = [0; 1024];

    let mut gz = super::WuffsGzipDecoder::new().unwrap();
    let (read, written) = gz.decode(&src, &mut dst).unwrap();

    assert_eq!(read, src.len());
    assert_eq!(written, cmp.len());

    let dst = &dst[..written];

    assert_eq!(dst, b"Hello Wuffs.\n");
  }
}
