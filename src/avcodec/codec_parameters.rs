use libav_sys_ng::{
    avcodec_get_name, avcodec_parameters_alloc, avcodec_parameters_copy, AVCodecParameters,
    AVMediaType, AVMediaType_AVMEDIA_TYPE_ATTACHMENT, AVMediaType_AVMEDIA_TYPE_AUDIO,
    AVMediaType_AVMEDIA_TYPE_DATA, AVMediaType_AVMEDIA_TYPE_NB, AVMediaType_AVMEDIA_TYPE_SUBTITLE,
    AVMediaType_AVMEDIA_TYPE_UNKNOWN, AVMediaType_AVMEDIA_TYPE_VIDEO,
};
use libav_sys_ng::{AVCodecID, AVColorRange, AVRational};
use std::ffi::CStr;
use std::fmt::Debug;

pub struct CodecParameters {
    pub(crate) _p: *mut AVCodecParameters,
}

impl CodecParameters {
    pub fn new() -> Option<CodecParameters> {
        unsafe {
            let raw = avcodec_parameters_alloc();

            if raw.is_null() {
                return None;
            }

            Some(CodecParameters { _p: raw })
        }
    }

    pub fn codec_type(&self) -> AVMediaType {
        unsafe { (*self._p).codec_type }
    }

    pub fn codec_id(&self) -> AVCodecID {
        unsafe { (*self._p).codec_id }
    }

    pub fn codec_name(&self) -> &str {
        unsafe {
            let id = (*self._p).codec_id;
            let name_raw = avcodec_get_name(id);

            if name_raw.is_null() {
                "(null)"
            } else {
                CStr::from_ptr(name_raw).to_str().unwrap()
            }
        }
    }

    pub fn bitrate(&self) -> i64 {
        unsafe { (*self._p).bit_rate }
    }

    pub fn size(&self) -> (i32, i32) {
        unsafe { ((*self._p).width, (*self._p).height) }
    }

    pub fn aspect_ratio(&self) -> AVRational {
        unsafe { (*self._p).sample_aspect_ratio }
    }

    pub fn framerate(&self) -> AVRational {
        unsafe { (*self._p).framerate }
    }

    pub fn color_range(&self) -> AVColorRange {
        unsafe { (*self._p).color_range }
    }

    pub fn format(&self) -> i32 {
        unsafe { (*self._p).format }
    }

    pub fn sample_rate(&self) -> Option<i32> {
        if self.codec_type() == AVMediaType_AVMEDIA_TYPE_AUDIO {
            Some(unsafe { (*self._p).sample_rate })
        } else {
            None
        }
    }

    #[inline]
    pub fn is_audio(&self) -> bool {
        self.codec_type() == AVMediaType_AVMEDIA_TYPE_AUDIO
    }

    #[inline]
    pub fn is_video(&self) -> bool {
        self.codec_type() == AVMediaType_AVMEDIA_TYPE_VIDEO
    }

    #[inline]
    pub fn raw(&self) -> *const AVCodecParameters {
        self._p.cast()
    }

    #[inline]
    pub fn raw_mut(&self) -> *mut AVCodecParameters {
        self._p
    }
}

impl Clone for CodecParameters {
    fn clone(&self) -> Self {
        let parameters = CodecParameters::new().expect("Failed to allocate AVCodecParameters");

        unsafe { avcodec_parameters_copy(parameters._p, self._p) };

        parameters
    }
}

impl Debug for CodecParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codec_type = unsafe {
            if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_NB {
                "AVMEDIA_TYPE_NB"
            } else if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_UNKNOWN {
                "AVMEDIA_TYPE_UNKNOWN"
            } else if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_AUDIO {
                "AVMEDIA_TYPE_AUDIO"
            } else if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_VIDEO {
                "AVMEDIA_TYPE_VIDEO"
            } else if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_DATA {
                "AVMEDIA_TYPE_DATA"
            } else if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_SUBTITLE {
                "AVMEDIA_TYPE_SUBTITLE"
            } else if (*self._p).codec_type == AVMediaType_AVMEDIA_TYPE_ATTACHMENT {
                "AVMEDIA_TYPE_ATTACHMENT"
            } else {
                "?"
            }
        };

        let (width, height) = self.size();

        f.debug_struct("CodecParameters")
            .field("codec_type", &codec_type)
            .field("codec_id", &self.codec_name())
            .field("bitrate", &self.bitrate())
            .field("width", &width)
            .field("height", &height)
            .field("aspect_ratio", &self.aspect_ratio())
            .field("frame_rate", &self.framerate())
            .finish()
    }
}

/*
impl Drop for CodecParameters {
    fn drop(&mut self) {
        unsafe { avcodec_parameters_free(&mut self._par) }
    }
}
*/
