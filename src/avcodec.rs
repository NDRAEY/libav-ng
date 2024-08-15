/// This module represents (almost) safe binding to AVCodecContext
use libav_sys_ng::{
    avcodec_alloc_context3, avcodec_find_encoder, avcodec_free_context, avcodec_is_open, avcodec_open2, avcodec_parameters_alloc, avcodec_parameters_copy, avcodec_parameters_free, avcodec_parameters_from_context, avcodec_parameters_to_context, avcodec_receive_packet, avcodec_send_frame, AVCodec, AVCodecContext, AVCodecID, AVCodecParameters, AVDictionary, AVPixelFormat, AVRational
};

use crate::{avdictionary::Dictionary, avframe, avstream::Stream};

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
    pub fn open(&mut self, options: Option<&mut Dictionary>) -> Result<(), i32> {
        let raw_options = match options {
            Some(opt) => (unsafe { &mut opt.raw() }) as *mut *mut AVDictionary,
            None => core::ptr::null_mut(),
        };
        let code = unsafe { avcodec_open2(self._codec_ctx, self._codec, raw_options) };

        if code < 0 {
            return Err(code);
        }

        Ok(())
    }

    /// Set size of codec picture size
    pub fn set_size(&mut self, width: i32, height: i32) -> &mut Self {
        unsafe {
            (*self._codec_ctx).width = width;
            (*self._codec_ctx).height = height;
        }

        self
    }

    /// Get size of codec picture size
    pub fn get_size(&self) -> (i32, i32) {
        unsafe { ((*self._codec_ctx).width, (*self._codec_ctx).height) }
    }

    /// Set bitrate
    pub fn set_bitrate(&mut self, bitrate: i64) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).bit_rate = bitrate;
        }

        self
    }

    /// Get bitrate
    pub fn get_bitrate(&self) -> i64 {
        unsafe {
            return (*self._codec_ctx).bit_rate;
        }
    }

    /// Set framerate (this also sets `time_base` but in inverse order)
    pub fn set_framerate(&mut self, fps: i32) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).time_base.num = 1;
            (*self._codec_ctx).time_base.den = fps;

            (*self._codec_ctx).framerate.num = fps;
            (*self._codec_ctx).framerate.den = 1;
        }

        self
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
    pub fn set_pixel_format(&mut self, fmt: AVPixelFormat) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).pix_fmt = fmt;
        }

        self
    }

    /// Get pixel format
    pub fn get_pixel_format(&self) -> AVPixelFormat {
        unsafe {
            return (*self._codec_ctx).pix_fmt;
        }
    }

    pub fn set_gop_size(&mut self, gop_size: i32) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).gop_size = gop_size;
        }

        self
    }

    pub fn get_gop_size(&self) -> i32 {
        unsafe {
            return (*self._codec_ctx).gop_size;
        }
    }

    pub fn set_max_b_frames(&mut self, max_b_frames: i32) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).max_b_frames = max_b_frames;
        }

        self
    }

    /// Fills parameters from codec into `params`
    pub fn fill_parameters(&self, params: &mut CodecParameters) {
        unsafe {
            avcodec_parameters_from_context(params._par, self._codec_ctx);
        }
    }

    /// Fills codec parameters from `params`
    pub fn fill_from_parameters(&self, params: &CodecParameters) {
        unsafe {
            avcodec_parameters_to_context(self._codec_ctx, params._par);
        }
    }

    pub fn fill_stream_parameters(&self, stream: &mut Stream) {
        unsafe {
            avcodec_parameters_from_context((*stream.raw()).codecpar, self._codec_ctx);
        }
    }

    /// Set codec flags
    pub fn set_flags(&mut self, flags: i32) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).flags = flags;
        }

        self
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
            avcodec_free_context(&mut self._codec_ctx);
        }
    }
}



pub struct CodecParameters {
    pub(crate) _par: *mut AVCodecParameters
}

impl CodecParameters {
    pub fn new() -> Option<CodecParameters> {
        unsafe {
            let raw = avcodec_parameters_alloc();
        
            if raw.is_null() {
                return None;
            }

            Some(CodecParameters { _par: raw })
        }
    }
}

impl Clone for CodecParameters {
    fn clone(&self) -> Self {
        let parameters = CodecParameters::new().expect("Failed to allocate AVCodecParameters");

        unsafe { avcodec_parameters_copy(parameters._par, self._par) };

        parameters
    }
}

impl Drop for CodecParameters {
    fn drop(&mut self) {
        unsafe {
            avcodec_parameters_free(&mut self._par)
        }
    }
}
