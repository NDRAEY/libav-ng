use std::cell::Cell;

use libav_sys_ng::{
    self, av_frame_alloc, av_frame_free, av_frame_get_buffer, av_get_bits_per_pixel,
    av_get_bytes_per_sample, av_pix_fmt_desc_get, AVFrame,
};

pub struct Frame {
    _frame: *mut libav_sys_ng::AVFrame,

    is_frame_allocated: bool,

    bytes_per_sample: Cell<Option<i32>>
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

            bytes_per_sample: Cell::new(None)
        })
    }

    pub fn from_size_and_pixfmt(width: i32, height: i32, pixfmt: i32) -> Option<Self> {
        unsafe {
            let mut _frame = av_frame_alloc();

            if _frame.is_null() {
                None
            } else {
                (*_frame).width = width;
                (*_frame).height = height;
                (*_frame).format = pixfmt;

                let mut this = Frame {
                    _frame,
                    is_frame_allocated: false,

                    bytes_per_sample: Cell::new(None)
                };

                this.allocate_buffer();

                Some(this)
            }
        }
    }

    pub fn data_plane(&self, plane_nr: usize) -> Result<&[u8], &str> {
        if plane_nr >= 8 {
            return Err("Out of bounds.");
        }

        if (unsafe { *self._frame }).data[plane_nr].is_null() {
            return Err("Invalid access.");
        }

        let size = self.frame_size()?;

        unsafe {
            Ok(core::slice::from_raw_parts(
                (*self._frame).data[plane_nr],
                size as usize,
            ))
        }
    }

    pub fn frame_size(&self) -> Result<usize, &str> {
        if self.is_audio() {
            return Ok(unsafe {
                (*self._frame).nb_samples as usize
                    * self.channel_layout().nb_channels as usize
                    // * av_get_bytes_per_sample((*self._frame).format) as usize
                    * self.bytes_per_sample() as usize
            });
        }

        let data = unsafe { av_pix_fmt_desc_get((*self._frame).format) };
        let depth = if data.is_null() {
            Err("Failed to get pixel format info")
        } else {
            Ok(unsafe { av_get_bits_per_pixel(data) } as usize)
        };

        if depth.is_err() {
            return Err(depth.err().unwrap());
        }

        let size = (unsafe { *self._frame }).width as usize
            * (unsafe { *self._frame }).height as usize
            * depth.ok().unwrap()
            / 8;

        Ok(size)
    }

    pub fn data_plane_mut(&mut self, plane_nr: usize) -> Result<&mut [u8], &str> {
        if plane_nr >= 8 {
            return Err("Out of bounds.");
        }

        let size = self.frame_size()?;

        unsafe {
            Ok(core::slice::from_raw_parts_mut(
                (*self._frame).data[plane_nr],
                size,
            ))
        }
    }

    fn allocate_buffer(&mut self) {
        if self.is_frame_allocated {
            return;
        }

        unsafe {
            av_frame_get_buffer(self._frame, 0);
        }

        self.is_frame_allocated = true;
    }

    #[inline(always)]
    pub fn format(&self) -> i32 {
        unsafe { (*self._frame).format }
    }

    #[inline(always)]
    pub fn linesize(&self) -> [i32; 8] {
        unsafe { (*self._frame).linesize }
    }

    #[inline(always)]
    pub fn sample_count(&self) -> i32 {
        unsafe { (*self._frame).nb_samples }
    }

    #[inline(always)]
    pub fn channel_layout(&self) -> &libav_sys_ng::AVChannelLayout {
        unsafe { &(*self._frame).ch_layout }
    }

    #[inline]
    pub fn is_audio(&self) -> bool {
        unsafe {
            (*self._frame).ch_layout.nb_channels != 0
                && (*self._frame).width == 0
                && (*self._frame).height == 0
        }
    }

    fn bytes_per_sample(&self) -> i32 {
        if self.bytes_per_sample.get().is_none() {
            self.bytes_per_sample.set(Some(unsafe { av_get_bytes_per_sample((*self._frame).format) }));
        }

        self.bytes_per_sample.get().unwrap()
    }

    #[inline(always)]
    pub fn pts(&self) -> i64 {
        unsafe { self.raw().pts }
    }

    #[inline(always)]
    pub unsafe fn raw(&self) -> &AVFrame {
        &*self._frame.cast()
    }

    #[inline(always)]
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
