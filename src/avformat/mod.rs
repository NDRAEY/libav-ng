use std::ffi::CString;

use libav_sys_ng::{
    self, AVFormatContext, AVInputFormat, AVOutputFormat, av_dump_format, av_read_frame, av_seek_frame, av_write_frame, av_write_trailer, avformat_alloc_output_context2, avformat_close_input, avformat_find_stream_info, avformat_free_context, avformat_open_input, avformat_write_header, avio_close, avio_open
};

use crate::{
    avdictionary::Dictionary, avformat::streams_iter::FormatStreamsIter, avpacket::Packet,
};

pub mod streams_iter;
pub struct FormatContext {
    _format_ctx: *mut libav_sys_ng::AVFormatContext,

    acquired_stream_info: bool,
    is_an_opened_input: bool,

    input_url: Option<CString>,
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
                &raw mut context,
                ptr,
                fmt_name.as_ptr(),
                real_filename.as_ptr(),
            );

            if context.is_null() {
                None
            } else {
                Some(FormatContext {
                    _format_ctx: context,

                    acquired_stream_info: false,
                    is_an_opened_input: false,

                    input_url: None,
                })
            }
        }
    }

    pub fn open_input(url: &str) -> Option<Self> {
        unsafe {
            let mut context = core::ptr::null_mut::<libav_sys_ng::AVFormatContext>();

            let url_c = CString::new(url).unwrap();

            // TODO: Manage InputFormat and Options arguments.
            let result = avformat_open_input(
                &raw mut context,
                url_c.as_ptr(),
                core::ptr::null(),
                core::ptr::null_mut(),
            );

            if result < 0 {
                None
            } else {
                Some(FormatContext {
                    _format_ctx: context,

                    acquired_stream_info: false,
                    is_an_opened_input: true,

                    input_url: Some(url_c),
                })
            }
        }
    }

    pub fn find_stream_info(&mut self) -> Option<i32> {
    	if self.acquired_stream_info {
    		return None;
    	}
    	
        let result = unsafe { avformat_find_stream_info(self._format_ctx, core::ptr::null_mut()) };
        
        println!("Streams: {result:?}");

        self.acquired_stream_info = true;

        Some(result)
    }

    pub unsafe fn get_input_format(&self) -> *const AVInputFormat {
        (*self._format_ctx).iformat
    }

    pub fn get_output_format(&self) -> &AVOutputFormat {
        unsafe { &*(*self._format_ctx).oformat as &AVOutputFormat }
    }

    pub unsafe fn raw(&self) -> &AVFormatContext {
        &*self._format_ctx
    }

    pub unsafe fn raw_mut(&mut self) -> &mut AVFormatContext {
        &mut *self._format_ctx
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

    pub fn seek_ts(&mut self, stream_idx: i32, timestamp: i64) -> i32 {
        unsafe { av_seek_frame(self._format_ctx, stream_idx, timestamp, 0) }
    }

    pub fn seek_msec(&mut self, stream_idx: i32, msec: i64) -> i32 {
        let tm = match self.streams().nth(stream_idx as _) {
            Some(st) => st.time_base(),
            None => return -1,
        };

        self.seek_ts(
            stream_idx,
            (msec as f64 * (tm.den as f64 / tm.num as f64) / 1000.0).floor() as _,
        )
    }

    pub fn read_frame(&mut self, packet: &mut Packet) -> i32 {
        packet.clear();

        unsafe { av_read_frame(self._format_ctx, packet.raw_mut()) }
    }

    pub fn write_frame(&mut self, packet: &mut Packet) -> i32 {
        unsafe { av_write_frame(self._format_ctx, packet.raw_mut()) }
    }

    pub fn streams(&mut self) -> FormatStreamsIter<'_> {
    	self.find_stream_info();
    	
        FormatStreamsIter::new(self)
    }

    pub fn write_trailer(&mut self) {
        unsafe { av_write_trailer(self._format_ctx) };
    }
}

impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            assert!(!self._format_ctx.is_null());

            // if !(*self._format_ctx).pb.is_null() {
            //     avio_close((*self._format_ctx).pb);
            // }

            if self.is_an_opened_input {
                avformat_close_input(&raw mut self._format_ctx);
            } else {
                avformat_free_context(self._format_ctx);
            }
        }
    }
}
