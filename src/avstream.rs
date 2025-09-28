use std::fmt::Debug;

use crate::{avcodec::codec_parameters::CodecParameters, avformat::FormatContext};
use libav_sys_ng::{AVCodec, AVRational, AVStream};

pub struct Stream {
    pub(crate) _stream: *mut libav_sys_ng::AVStream,
}

impl Stream {
    pub fn new(format_ctx: &mut FormatContext, codec: Option<&AVCodec>) -> Option<Stream> {
        unsafe {
            let raw_fc = format_ctx.raw_mut();

            let raw_codec = match codec {
                Some(c) => c as *const AVCodec,
                None => core::ptr::null::<AVCodec>(),
            };

            let stream = libav_sys_ng::avformat_new_stream(raw_fc, raw_codec);

            if stream.is_null() {
                return None;
            }

            Some(Stream { _stream: stream })
        }
    }

    pub fn codec_parameters(&self) -> CodecParameters {
        unsafe {
            CodecParameters {
                _p: (*self._stream).codecpar,
            }
        }
    }

    pub fn id(&self) -> i32 {
        unsafe { (*self._stream).id }
    }

    pub fn index(&self) -> i32 {
        unsafe { (*self._stream).index }
    }

    pub fn time_base(&self) -> AVRational {
        unsafe { (*self._stream).time_base }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self._stream).duration }
    }

    pub fn duration_sec(&self) -> f64 {
        let time_base = self.time_base();
        let duration = self.duration();

        (duration as f64 * time_base.num as f64) / time_base.den as f64
    }

    pub unsafe fn raw(&self) -> &AVStream {
        &*self._stream.cast()
    }

    pub unsafe fn raw_mut(&mut self) -> &mut AVStream {
        &mut *self._stream
    }
}

impl Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let index = (*self._stream).index;
            let id = (*self._stream).id;
            let codec_parameters = CodecParameters {
                _p: (*self._stream).codecpar,
            };

            let time_base = self.time_base();
            let start_time = (*self._stream).start_time;
            let duration = self.duration();

            let nb_frames = (*self._stream).nb_frames;

            let avg_frame_rate = (*self._stream).avg_frame_rate;
            let r_frame_rate = (*self._stream).r_frame_rate;

            f.debug_struct("Stream")
                .field("id", &id)
                .field("index", &index)
                .field("codecpar", &codec_parameters)
                .field("time_base", &time_base)
                .field("start_time", &start_time)
                .field("nb_frames", &nb_frames)
                .field("duration", &duration)
                .field("avg_frame_rate", &avg_frame_rate)
                .field("r_frame_rate", &r_frame_rate)
                .finish()
        }
    }
}
