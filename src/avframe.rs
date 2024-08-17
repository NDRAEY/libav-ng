use libav_sys_ng::{
    self, av_frame_alloc, av_frame_free, av_image_alloc, av_pix_fmt_desc_get, AVPixFmtDescriptor,
    AVPixelFormat_AV_PIX_FMT_RGB48BE,
};

pub struct Frame {
    _frame: *mut libav_sys_ng::AVFrame,
}

impl Frame {
    pub fn from_size_and_pixfmt(width: i32, height: i32, pixfmt: i32) -> Option<Frame> {
        unsafe {
            let mut _frame = av_frame_alloc();

            if _frame == core::ptr::null_mut() {
                return None;
            } else {
                (*_frame).width = width;
                (*_frame).height = height;
                (*_frame).format = pixfmt;

                let val = av_image_alloc(
                    (*_frame).data.as_mut_ptr(),
                    (*_frame).linesize.as_mut_ptr(),
                    width,
                    height,
                    pixfmt,
                    1,
                );

                if val < 0 {
                    av_frame_free(&mut _frame);
                    return None;
                }

                Some(Frame { _frame })
            }
        }
    }

    pub fn data_plane(&mut self, plane_nr: usize) -> Result<&mut [u8], &str> {
        unsafe {
            if plane_nr >= 8 {
                return Err("Out of bounds.");
            }

            let data = av_pix_fmt_desc_get((*self._frame).format);
            let depth = if data.is_null() {
                Err("Failed to get pixel format info")
            } else {
                Ok((*data).nb_components as usize * 8)
            };

            if depth.is_err() {
                return Err(depth.err().unwrap());
            }

            return Ok(core::slice::from_raw_parts_mut(
                (*self._frame).data[plane_nr],
                (*self._frame).width as usize * (*self._frame).height as usize * depth.ok().unwrap(),
            ));
        }
    }

    pub(crate) unsafe fn raw(&mut self) -> *mut libav_sys_ng::AVFrame {
        return self._frame;
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            av_frame_free(&mut self._frame);
        }
    }
}
