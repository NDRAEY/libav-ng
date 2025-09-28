use libav_sys_ng::{sws_freeContext, sws_getContext, sws_scale, SwsContext};

use crate::avframe::Frame;

pub struct Sws {
    inner: *mut SwsContext,
    src_width: i32,
    src_height: i32,
    src_format: i32,
    dst_width: i32,
    dst_height: i32,
    dst_format: i32,
    flags: i32,
}

impl Sws {
    pub fn new(
        src_width: i32,
        src_height: i32,
        src_format: i32,
        dst_width: i32,
        dst_height: i32,
        dst_format: i32,
        flags: i32,
    ) -> Self {
        Sws {
            inner: unsafe {
                sws_getContext(
                    src_width,
                    src_height,
                    src_format,
                    dst_width,
                    dst_height,
                    dst_format,
                    flags,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    core::ptr::null(),
                )
            },
            src_width,
            src_height,
            src_format,
            dst_width,
            dst_height,
            dst_format,
            flags,
        }
    }

    pub fn scale(&self, in_frame: &Frame, out_frame: &mut Frame) -> i32 {
        let src_slice = (unsafe { in_frame.raw() }).data.as_ptr().cast();
        let src_stride = in_frame.linesize();

        let dst_slice = unsafe { out_frame.raw_mut().data.as_ptr() };
        let dst_stride = out_frame.linesize();

        unsafe {
            sws_scale(
                self.inner,
                src_slice,
                src_stride.as_ptr(),
                0,
                self.src_height,
                dst_slice,
                dst_stride.as_ptr(),
            )
        }
    }
}

impl Drop for Sws {
    fn drop(&mut self) {
        unsafe { sws_freeContext(self.inner) };
    }
}
