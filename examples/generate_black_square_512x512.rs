use libav_ng::{self, avcodec::CodecContext, avformat::FormatContext, avframe::Frame, avpacket::Packet, avstream::Stream, low_level};
use libav_sys_ng::{AVPixelFormat_AV_PIX_FMT_RGB24, AVIO_FLAG_WRITE};

fn main() {
    let filename = "bk.png";
    let width = 512;
    let height = 512;

    let mut format_ctx =
        FormatContext::new("image2", filename, None).expect("Failed to create FormatContext");

    let mut codec = CodecContext::from_encoder_id(low_level::AVCodecID_AV_CODEC_ID_PNG)
        .expect("Failed to create CodecContext");

    let mut stream =
        Stream::new(&mut format_ctx, Some(&*codec)).expect("Failed to create a stream");

    codec
        .set_size(width, height)
        .set_framerate(1)
        .set_pixel_format(low_level::AVPixelFormat_AV_PIX_FMT_RGB24)
        .set_bitrate(400_000)
        .set_gop_size(1)
        .set_max_b_frames(1);

    if (format_ctx.get_output_format().flags & low_level::AVFMT_GLOBALHEADER as i32) != 0 {
        let flags = codec.get_flags() | low_level::AV_CODEC_FLAG_GLOBAL_HEADER as i32;

        codec.set_flags(flags);
    }

    match codec.open(None) {
        Err(err) => panic!("Error opening codec: {}", err),
        Ok(()) => {}
    };

    codec.fill_stream_parameters(&mut stream);

    match format_ctx.open(filename, AVIO_FLAG_WRITE as i32) {
        Err(err) => panic!("Error opening file! {err}"),
        Ok(()) => {}
    };

    format_ctx
        .write_header(None)
        .expect("Failed to write a header!");

    let mut frame = Frame::from_size_and_pixfmt(width, height, AVPixelFormat_AV_PIX_FMT_RGB24)
        .expect("Failed to make a frame!");

    let data = frame.data_plane_mut(0).expect("Failed to get plane!");

    for y in 0usize..height as usize {
        for x in 0usize..width as usize {
            let coord = y * (width as usize * 3usize) + (x * 3);

            data[coord + 0] = 0xff;
            data[coord + 1] = 0xff;
            data[coord + 2] = 0xff;
        }
    }

    codec.send_frame(&frame);

    let mut packet = Packet::new();

    codec.receive_packet(&mut packet);

    format_ctx.write_frame(&mut packet);

    packet.clear();

    format_ctx.write_trailer();

    println!("Hello, world!");
}
