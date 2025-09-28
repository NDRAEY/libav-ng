use libav_sys_ng::{av_packet_alloc, av_packet_free, av_packet_unref, AVPacket};

pub struct Packet {
    pub(crate) inner: *mut AVPacket,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            inner: unsafe { av_packet_alloc() },
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            av_packet_unref(self.inner);
        }
    }

    pub fn stream_index(&self) -> i32 {
        unsafe { (*self.inner).stream_index }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts((*self.inner).data, (*self.inner).size as _) }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.inner).duration }
    }

    pub fn position(&self) -> i64 {
        unsafe { (*self.inner).pos }
    }

    pub unsafe fn raw(&self) -> *const AVPacket {
        self.inner.cast()
    }

    pub unsafe fn raw_mut(&mut self) -> *mut AVPacket {
        self.inner
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe { av_packet_free(&mut self.inner) };
    }
}
