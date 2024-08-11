use libav_sys_ng::{self, avformat_alloc_output_context2, avformat_free_context, AVOutputFormat};

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

            avformat_alloc_output_context2(
                &mut context,
                ptr,
                format_name.as_bytes().as_ptr() as *const i8,
                filename.as_bytes().as_ptr() as *const i8,
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
}

impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            avformat_free_context(self._format_ctx);
        }
    }
}
