use libav_sys_ng::{
    avcodec_alloc_context3, avcodec_find_encoder, avcodec_free_context, avcodec_is_open,
    avcodec_open2, avcodec_parameters_from_context, avcodec_receive_packet, avcodec_send_frame,
    AVCodec, AVCodecContext, AVCodecID, AVCodecParameters, AVDictionary, AVPixelFormat, AVRational,
};

use crate::avframe;

pub struct CodecContext {
    _codec: *const AVCodec,
    _codec_ctx: *mut AVCodecContext,
}

impl CodecContext {
    pub fn from_encoder_id(id: AVCodecID) -> Option<CodecContext> {
        unsafe {
            let codec = avcodec_find_encoder(id);

            if codec == core::ptr::null() {
                return None;
            }

            let codec_ctx = avcodec_alloc_context3(codec);

            if codec_ctx == core::ptr::null_mut() {
                return None;
            }

            Some(CodecContext {
                _codec: codec,
                _codec_ctx: codec_ctx,
            })
        }
    }

    pub fn open(&mut self, options: *mut *mut AVDictionary) -> i32 {
        unsafe { avcodec_open2(self._codec_ctx, self._codec, options) }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe {
            (*self._codec_ctx).width = width;
            (*self._codec_ctx).height = height;
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        unsafe { ((*self._codec_ctx).width, (*self._codec_ctx).height) }
    }

    pub fn set_bitrate(&mut self, bitrate: i64) {
        unsafe {
            (*self._codec_ctx).bit_rate = bitrate;
        }
    }

    pub fn get_bitrate(&self) -> i64 {
        unsafe {
            return (*self._codec_ctx).bit_rate;
        }
    }

    pub fn set_framerate(&mut self, fps: i32) {
        unsafe {
            (*self._codec_ctx).time_base.num = 1;
            (*self._codec_ctx).time_base.den = fps;

            (*self._codec_ctx).framerate.num = fps;
            (*self._codec_ctx).framerate.den = 1;
        }
    }

    pub fn get_framerate(&self) -> AVRational {
        unsafe {
            return (*self._codec_ctx).framerate;
        }
    }

    pub fn get_time_base(&self) -> AVRational {
        unsafe {
            return (*self._codec_ctx).time_base;
        }
    }

    pub fn set_pixel_format(&mut self, fmt: AVPixelFormat) {
        unsafe {
            (*self._codec_ctx).pix_fmt = fmt;
        }
    }

    pub fn get_pixel_format(&self) -> AVPixelFormat {
        unsafe {
            return (*self._codec_ctx).pix_fmt;
        }
    }

    pub fn fill_parameters(&self, params: *mut AVCodecParameters) {
        unsafe {
            avcodec_parameters_from_context(params, self._codec_ctx);
        }
    }

    pub fn set_flags(&mut self, flags: i32) {
        unsafe {
            (*self._codec_ctx).flags = flags;
        }
    }

    pub fn get_flags(&self) -> i32 {
        unsafe {
            return (*self._codec_ctx).flags;
        }
    }

    pub fn send_frame(&mut self, frame: &mut avframe::Frame) -> i32 {
        unsafe {
            return avcodec_send_frame(self._codec_ctx, frame.raw());
        }
    }

    // TODO: Make this function return Packet when it gets implemented.
    pub unsafe fn receive_packet(&mut self, out: *mut libav_sys_ng::AVPacket) -> i32 {
        return avcodec_receive_packet(self._codec_ctx, out);
    }

    pub fn is_open(&self) -> bool {
        unsafe {
            return avcodec_is_open(self._codec_ctx) != 0;
        }
    }
}

impl Drop for CodecContext {
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self._codec_ctx as *mut *mut _);
        }
    }
}
