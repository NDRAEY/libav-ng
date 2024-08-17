use std::ffi::{CStr, CString};

use libav_sys_ng::{
    self, av_dump_format, avformat_alloc_output_context2, avformat_free_context,
    avformat_write_header, avio_open, AVFormatContext, AVInputFormat, AVOutputFormat,
};

use crate::avdictionary::Dictionary;

pub struct FormatContext {
    _format_ctx: *mut libav_sys_ng::AVFormatContext,
}

impl FormatContext {
    pub fn new(
        format_name: &str,
        filename: &str,
        output_format: Option<AVOutputFormat>,
    ) -> Option<FormatContext> {
        unsafe {
            let mut context = core::ptr::null_mut::<libav_sys_ng::AVFormatContext>();

            let ptr = match output_format {
                Some(fmt) => &fmt,
                None => core::ptr::null::<AVOutputFormat>(),
            };

            let fmt_name = CString::new(format_name).expect("CString::new(format_name) failed");
            let real_filename = CString::new(filename).expect("CString::new(filename) failed");

            avformat_alloc_output_context2(
                &mut context,
                ptr,
                fmt_name.as_ptr(),
                real_filename.as_ptr(),
            );

            if context == core::ptr::null_mut() {
                return None;
            } else {
                return Some(FormatContext {
                    _format_ctx: context,
                });
            }
        }
    }

    pub unsafe fn get_input_format(&self) -> *const AVInputFormat {
        return (*self._format_ctx).iformat;
    }

    pub fn get_output_format(&self) -> &AVOutputFormat {
        unsafe {
            return &*(*self._format_ctx).oformat as &AVOutputFormat;
        }
    }

    pub unsafe fn raw(&mut self) -> *mut AVFormatContext {
        self._format_ctx
    }

    pub fn dump(&self, index: i32, url: &str, is_output: bool) {
        unsafe {
            let raw_url = CString::new(url).expect("CString::new(url) failed");

            av_dump_format(self._format_ctx, index, raw_url.as_ptr(), is_output as i32)
        }
    }

    pub fn open(&mut self, url: &str, flags: i32) -> Result<(), i32> {
        unsafe {
            let raw_url = CString::new(url).expect("CString::new(url) failed");

            let x = avio_open(&mut (*self._format_ctx).pb, raw_url.as_ptr(), flags);

            if x < 0 {
                return Err(x);
            }
        }

        Ok(())
    }

    pub fn write_header(&mut self, options: Option<&mut Dictionary>) -> Result<(), i32> {
        unsafe {
            let raw_options = match options {
                Some(op) => &mut op.raw(),
                None => core::ptr::null_mut(),
            };

            let code = avformat_write_header(self._format_ctx, raw_options);

            if code < 0 {
                return Err(code);
            }
        }

        Ok(())
    }
}

impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            avformat_free_context(self._format_ctx);
        }
    }
}
