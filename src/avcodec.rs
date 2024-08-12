/// This module represents (almost) safe binding to AVCodecContext

use libav_sys_ng::{
    avcodec_alloc_context3, avcodec_find_encoder, avcodec_free_context, avcodec_is_open,
    avcodec_open2, avcodec_parameters_from_context, avcodec_receive_packet, avcodec_send_frame,
    AVCodec, AVCodecContext, AVCodecID, AVCodecParameters, AVDictionary, AVPixelFormat, AVRational,
};

use crate::avframe;

/// AVCodecContext wrapper
pub struct CodecContext {
    _codec: *const AVCodec,
    _codec_ctx: *mut AVCodecContext,
}

impl CodecContext {
    /// Creates CodecContext from encoder ID
    /// 
    /// Returns Some(CodecContext) on success, None on error.
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

    /// Opens a codec
    ///
    /// Returns 0 on success, negative number (Linux error code) on error
    ///
    /// # TODO
    ///
    /// Wrap it into Result<T, A>
    pub fn open(&mut self, options: *mut *mut AVDictionary) -> i32 {
        unsafe { avcodec_open2(self._codec_ctx, self._codec, options) }
    }

    /// Set size of codec picture size
    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe {
            (*self._codec_ctx).width = width;
            (*self._codec_ctx).height = height;
        }
    }

    /// Get size of codec picture size
    pub fn get_size(&self) -> (i32, i32) {
        unsafe { ((*self._codec_ctx).width, (*self._codec_ctx).height) }
    }

    /// Set bitrate
    pub fn set_bitrate(&mut self, bitrate: i64) {
        unsafe {
            (*self._codec_ctx).bit_rate = bitrate;
        }
    }

    /// Get bitrate
    pub fn get_bitrate(&self) -> i64 {
        unsafe {
            return (*self._codec_ctx).bit_rate;
        }
    }

    /// Set framerate (this also sets `time_base` but in inverse order)
    pub fn set_framerate(&mut self, fps: i32) {
        unsafe {
            (*self._codec_ctx).time_base.num = 1;
            (*self._codec_ctx).time_base.den = fps;

            (*self._codec_ctx).framerate.num = fps;
            (*self._codec_ctx).framerate.den = 1;
        }
    }

    /// Get framerate
    pub fn get_framerate(&self) -> AVRational {
        unsafe {
            return (*self._codec_ctx).framerate;
        }
    }

    /// Get time base
    pub fn get_time_base(&self) -> AVRational {
        unsafe {
            return (*self._codec_ctx).time_base;
        }
    }

    /// Set pixel format
    pub fn set_pixel_format(&mut self, fmt: AVPixelFormat) {
        unsafe {
            (*self._codec_ctx).pix_fmt = fmt;
        }
    }

    /// Get pixel format
    pub fn get_pixel_format(&self) -> AVPixelFormat {
        unsafe {
            return (*self._codec_ctx).pix_fmt;
        }
    }

    /// Fills parameters from codec into `params`
    pub fn fill_parameters(&self, params: *mut AVCodecParameters) {
        unsafe {
            avcodec_parameters_from_context(params, self._codec_ctx);
        }
    }

    /// Set codec flags
    pub fn set_flags(&mut self, flags: i32) {
        unsafe {
            (*self._codec_ctx).flags = flags;
        }
    }

    /// Get codec flags
    pub fn get_flags(&self) -> i32 {
        unsafe {
            return (*self._codec_ctx).flags;
        }
    }

    /// Send frame to codec
    pub fn send_frame(&mut self, frame: &mut avframe::Frame) -> i32 {
        unsafe {
            return avcodec_send_frame(self._codec_ctx, frame.raw());
        }
    }

    // TODO: Make this function return Packet when it gets implemented.
    /// Receive packet from codec to `out`
    pub unsafe fn receive_packet(&mut self, out: *mut libav_sys_ng::AVPacket) -> i32 {
        return avcodec_receive_packet(self._codec_ctx, out);
    }

    /// Returns true if codec is opened.
    pub fn is_open(&self) -> bool {
        unsafe {
            return avcodec_is_open(self._codec_ctx) != 0;
        }
    }
}

impl Drop for CodecContext {
    /// Frees context on drop
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self._codec_ctx as *mut *mut _);
        }
    }
}
