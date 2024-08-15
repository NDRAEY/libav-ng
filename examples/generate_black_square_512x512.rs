use libav_ng::{self, avcodec::CodecContext, avformat::FormatContext, low_level};

fn main() {
    let filename = "bk.png";
    let width = 512;
    let height = 512;

    let mut format_ctx =
        FormatContext::new("image2", filename, None).expect("Failed to create FormatContext");
    let mut codec = CodecContext::from_encoder_id(low_level::AVCodecID_AV_CODEC_ID_PNG)
        .expect("Failed to create CodecContext");

    let oformat = format_ctx.get_output_format();

    let stream = libav_ng::avstream::Stream::new(&mut format_ctx, None);

    codec.set_size(width, height);
    codec.set_framerate(1);
    codec.set_pixel_format(low_level::AVPixelFormat_AV_PIX_FMT_RGB24);
    codec.set_bitrate(400_000);
    codec.set_gop_size(1);
    codec.set_max_b_frames(1);

    if (format_ctx.get_output_format().flags & low_level::AVFMT_GLOBALHEADER as i32) != 0 {
        codec.set_flags(codec.get_flags() | low_level::AV_CODEC_FLAG_GLOBAL_HEADER as i32);
    }

    println!("Hello, world!");
}
