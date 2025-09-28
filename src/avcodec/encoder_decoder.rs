/// Structures `Encoder` and `Decoder` provided by this module are used to differ
/// decoder and encoder in CodecContext, because using wrong mode cause SIGSEGV.use std::ops::{Deref, DerefMut};
use std::ops::{Deref, DerefMut};

use libav_sys_ng::{avcodec_receive_frame, avcodec_send_frame};

use crate::{avcodec::CodecContext, avframe::Frame};

pub struct Encoder {
    pub(crate) ctx: CodecContext,
}

impl Encoder {
    /// Send frame to codec
    pub fn send_frame(&mut self, frame: &Frame) -> i32 {
        unsafe { avcodec_send_frame(self._codec_ctx, frame.raw()) }
    }
}

impl Deref for Encoder {
    type Target = CodecContext;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub struct Decoder {
    pub(crate) ctx: CodecContext,
}

impl Decoder {
    pub fn receive_frame(&mut self, out: &mut Frame) -> i32 {
        unsafe { avcodec_receive_frame(self._codec_ctx, out.raw_mut()) }
    }
}

impl Deref for Decoder {
    type Target = CodecContext;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}
