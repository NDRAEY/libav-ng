use libav_sys_ng::{self, av_frame_alloc, av_frame_free};

pub struct Frame {
    _frame: *mut libav_sys_ng::AVFrame,
}

impl Frame {
    pub fn from_size(width: i32, height: i32) -> Option<Frame> {
        unsafe {
            let _frame = av_frame_alloc();

            if _frame == core::ptr::null_mut() {
                return None;
            } else {
                (*_frame).width = width;
                (*_frame).height = height;

                Some(Frame { _frame })
            }
        }
    }

    pub fn from_size_and_pixfmt(width: i32, height: i32, pixfmt: i32) -> Option<Frame> {
        unsafe {
            let _frame = av_frame_alloc();

            if _frame == core::ptr::null_mut() {
                return None;
            } else {
                (*_frame).width = width;
                (*_frame).height = height;
                (*_frame).format = pixfmt;

                Some(Frame { _frame })
            }
        }
    }

    pub unsafe fn raw(&mut self) -> *mut libav_sys_ng::AVFrame {
        return self._frame;
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            av_frame_free(&mut self._frame as _);
        }
    }
}
