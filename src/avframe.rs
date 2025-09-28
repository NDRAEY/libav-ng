use libav_sys_ng::{
    self, av_frame_alloc, av_frame_free, av_frame_get_buffer, av_get_bits_per_pixel,
    av_get_padded_bits_per_pixel, av_image_alloc, av_pix_fmt_desc_get, AVFrame,
};

pub struct Frame {
    _frame: *mut libav_sys_ng::AVFrame,
    is_frame_allocated: bool,
}

impl Frame {
    pub fn empty() -> Option<Self> {
        let _frame = unsafe { av_frame_alloc() };

        if _frame.is_null() {
            return None;
        }

        Some(Self {
            _frame,
            is_frame_allocated: false,
        })
    }

    pub fn from_size_and_pixfmt(width: i32, height: i32, pixfmt: i32) -> Option<Self> {
        unsafe {
            let mut _frame = av_frame_alloc();

            if _frame.is_null() {
                return None;
            } else {
                (*_frame).width = width;
                (*_frame).height = height;
                (*_frame).format = pixfmt;

                Some(Frame {
                    _frame,
                    is_frame_allocated: false,
                })
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
                Ok(av_get_bits_per_pixel(data) as usize)
            };

            if depth.is_err() {
                return Err(depth.err().unwrap());
            }

            let size = (*self._frame).width as usize
                * (*self._frame).height as usize
                * depth.ok().unwrap()
                / 8;

            return Ok(core::slice::from_raw_parts_mut(
                (*self._frame).data[plane_nr],
                size,
            ));
        }
    }

    pub fn allocate_buffer(&mut self) {
        if self.is_frame_allocated {
            return;
        }

        unsafe {
            av_frame_get_buffer(self._frame, 0);
        }

        self.is_frame_allocated = true;
    }

    pub fn format(&self) -> i32 {
        unsafe { (*self._frame).format }
    }

    pub fn linesize(&self) -> [i32; 8] {
        unsafe { (*self._frame).linesize }
    }

    pub unsafe fn raw(&self) -> &AVFrame {
        &*self._frame.cast()
    }

    pub unsafe fn raw_mut(&mut self) -> &mut AVFrame {
        &mut *self._frame
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            av_frame_free(&mut self._frame);
        }
    }
}
