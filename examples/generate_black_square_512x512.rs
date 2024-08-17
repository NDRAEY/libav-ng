use libav_ng::{self, avcodec::CodecContext, avformat::FormatContext, avframe::Frame, low_level};
use libav_sys_ng::{AVPixelFormat_AV_PIX_FMT_RGB24, AVIO_FLAG_WRITE};

fn main() {
    let filename = "bk.png";
    let width = 512;
    let height = 512;

    let mut format_ctx =
        FormatContext::new("image2", filename, None).expect("Failed to create FormatContext");
    let mut codec = CodecContext::from_encoder_id(low_level::AVCodecID_AV_CODEC_ID_PNG)
        .expect("Failed to create CodecContext");

    let oformat = format_ctx.get_output_format();

    let mut stream =
        libav_ng::avstream::Stream::new(&mut format_ctx, None).expect("Failed to create a stream");

    codec
        .set_size(width, height)
        .set_framerate(1)
        .set_pixel_format(low_level::AVPixelFormat_AV_PIX_FMT_RGB24)
        .set_bitrate(400_000)
        .set_gop_size(1)
        .set_max_b_frames(1);

    if (format_ctx.get_output_format().flags & low_level::AVFMT_GLOBALHEADER as i32) != 0 {
        codec.set_flags(codec.get_flags() | low_level::AV_CODEC_FLAG_GLOBAL_HEADER as i32);
    }

    match codec.open(None) {
        Err(err) => panic!("Error opening codec: {}", err),
        Ok(()) => {}
    };

    codec.fill_stream_parameters(&mut stream);


    match format_ctx.open("black.png", AVIO_FLAG_WRITE as i32) {
        Err(err) => panic!("Error opening file! {err}"),
        Ok(()) => {}
    };

    format_ctx.write_header(None).expect("Failed to write a header!");


    let mut frame = Frame::from_size_and_pixfmt(width, height, AVPixelFormat_AV_PIX_FMT_RGB24).expect("Failed to make a frame!");

    let mut data = frame.data_plane(0).expect("Failed to get plane!");

    for y in 0usize..height as usize {
        for x in 0usize..width as usize {
            data[y * (width as usize * 3usize) + x] = 0xff;
        }
    }

    println!("Hello, world!");
}
