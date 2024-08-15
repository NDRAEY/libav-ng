use crate::avformat::FormatContext;
use libav_sys_ng::AVCodec;

pub struct Stream {
    _stream: *mut libav_sys_ng::AVStream,
}

impl Stream {
    pub fn new(format_ctx: &mut FormatContext, codec: Option<&AVCodec>) -> Option<Stream> {
        unsafe {
            let raw_fc = format_ctx.raw();

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
}
