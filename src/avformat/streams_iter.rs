
use crate::{avformat::FormatContext, avstream::Stream};

pub struct FormatStreamsIter<'a> {
    position: usize,
    count: usize,
    format_ctx: &'a mut FormatContext,
}

impl<'a> FormatStreamsIter<'a> {
    pub fn new(format_ctx: &'a mut FormatContext) -> Self {
        let count = unsafe { format_ctx.raw().nb_streams } as usize;

        Self {
            position: 0,
            count,
            format_ctx,
        }
    }
}

impl Iterator for FormatStreamsIter<'_> {
    type Item = Stream;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.count {
            return None;
        }

        let stream = unsafe { self.format_ctx.raw().streams.add(self.position).read() };

        self.position += 1;

        Some(Stream { _stream: stream })
    }
}
