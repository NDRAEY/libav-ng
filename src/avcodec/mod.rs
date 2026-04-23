/// This module represents (almost) safe binding to AVCodecContext
use libav_sys_ng::{
    avcodec_alloc_context3, avcodec_find_decoder, avcodec_find_encoder, avcodec_free_context,
    avcodec_is_open, avcodec_open2, avcodec_parameters_from_context, avcodec_parameters_to_context,
    avcodec_receive_packet, avcodec_send_packet, AVCodec, AVCodecContext, AVCodecID, AVDictionary,
    AVPixelFormat, AVRational,
};

use crate::{
    avcodec::{
        codec_parameters::CodecParameters,
        encoder_decoder::{Decoder, Encoder},
    },
    avdictionary::Dictionary,
    avpacket::Packet,
    avstream::Stream,
};

pub mod codec_parameters;
pub mod encoder_decoder;
pub mod error;

/// AVCodecContext wrapper
pub struct CodecContext {
    _codec: *const AVCodec,
    _codec_ctx: *mut AVCodecContext,
}

impl CodecContext {
    /// Creates CodecContext from encoder ID
    ///
    /// Returns Some(CodecContext) on success, None on error.
    pub fn from_encoder_id(id: AVCodecID) -> Option<Encoder> {
        unsafe {
            let codec = avcodec_find_encoder(id);

            if codec.is_null() {
                return None;
            }

            let codec_ctx = avcodec_alloc_context3(codec);

            if codec_ctx.is_null() {
                return None;
            }

            Some(Encoder {
                ctx: CodecContext {
                    _codec: codec,
                    _codec_ctx: codec_ctx,
                },
            })
        }
    }

    pub fn from_decoder_id(id: AVCodecID) -> Option<Decoder> {
        unsafe {
            let codec = avcodec_find_decoder(id);

            if codec.is_null() {
                return None;
            }

            let codec_ctx = avcodec_alloc_context3(codec);

            if codec_ctx.is_null() {
                return None;
            }

            Some(Decoder {
                ctx: CodecContext {
                    _codec: codec,
                    _codec_ctx: codec_ctx,
                },
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
        unsafe { (*self._codec_ctx).bit_rate }
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
        unsafe { (*self._codec_ctx).framerate }
    }

    /// Get time base
    pub fn get_time_base(&self) -> AVRational {
        unsafe { (*self._codec_ctx).time_base }
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
        unsafe { (*self._codec_ctx).pix_fmt }
    }

    pub fn set_gop_size(&mut self, gop_size: i32) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).gop_size = gop_size;
        }

        self
    }

    pub fn get_gop_size(&self) -> i32 {
        unsafe { (*self._codec_ctx).gop_size }
    }

    pub fn set_max_b_frames(&mut self, max_b_frames: i32) -> &mut CodecContext {
        unsafe {
            (*self._codec_ctx).max_b_frames = max_b_frames;
        }

        self
    }

    pub fn sample_format(&self) -> i32 {
        (unsafe { *self._codec_ctx }).sample_fmt
    }

    /// Fills parameters from codec into `params`
    pub fn fill_parameters(&self, params: &mut CodecParameters) {
        unsafe {
            avcodec_parameters_from_context(params._p, self._codec_ctx);
        }
    }

    /// Fills codec parameters from `params`
    pub fn fill_from_parameters(&self, params: &CodecParameters) {
        unsafe {
            avcodec_parameters_to_context(self._codec_ctx, params._p);
        }
    }

    pub fn fill_stream_parameters(&self, stream: &Stream) {
        unsafe {
            avcodec_parameters_from_context(stream.raw().codecpar, self._codec_ctx);
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
        unsafe { (*self._codec_ctx).flags }
    }

    /// Receive packet from codec to `out`
    pub fn receive_packet(&mut self, out: &mut Packet) -> i32 {
        unsafe { avcodec_receive_packet(self._codec_ctx, out.raw_mut()) }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> i32 {
        unsafe { avcodec_send_packet(self._codec_ctx, packet.raw()) }
    }

    /// Returns true if codec is opened.
    pub fn is_open(&self) -> bool {
        unsafe { avcodec_is_open(self._codec_ctx) != 0 }
    }

    pub unsafe fn raw_codec(&self) -> *const AVCodec {
        self._codec
    }

    pub unsafe fn raw_codec_context(&self) -> *const AVCodecContext {
        self._codec_ctx
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
