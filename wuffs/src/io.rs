// use crate::types::WuffsBox;
// use wuffs_sys::{wuffs_base__io_buffer, wuffs_base__io_buffer_meta};

// #[derive(Copy, Clone)]
// pub struct WuffsIoBuffer(wuffs_base__io_buffer);

// #[derive(Copy, Clone)]
// pub struct WuffsIoBufferMeta(wuffs_base__io_buffer_meta);

// impl WuffsIoBuffer {
//   pub fn data(&self) -> WuffsSlice {
//     WuffsSlice(self.0.data)
//   }

//   pub fn meta(&self) -> WuffsIoBufferMeta {
//     WuffsIoBufferMeta(self.0.meta)
//   }
// }

// impl WuffsIoBufferMeta {
//   pub fn pos(&self) -> usize {
//     self.0.pos as _
//   }

//   pub fn closed(&self) -> bool {
//     self.0.closed
//   }

//   pub fn write_increment(&self) -> usize {
//     self.0.wi as _
//   }

//   pub fn read_increment(&self) -> usize {
//     self.0.ri as _
//   }

//   pub fn into_inner(self) -> wuffs_base__io_buffer_meta {
//     self.0
//   }
// }
