use std::io::Write;

use libav_ng::{
    avcodec::CodecContext, avformat::FormatContext, avframe::Frame, avpacket::Packet, sws::Sws,
};
use libav_sys_ng::{
    sws_freeContext, sws_getContext, sws_scale, AVPixelFormat_AV_PIX_FMT_RGB24, SwsContext,
    SWS_BILINEAR,
};

fn main() {
    let url = if let Some(url) = std::env::args().skip(1).next() {
        url
    } else {
        eprintln!("No input file!");
        std::process::exit(1);
    };

    let mut format_context = FormatContext::open_input(&url).expect("Failed to open file!");

    format_context.find_stream_info();

    for i in format_context.streams() {
        println!("{i:#?}");
    }

    let video_stream = format_context
        .streams()
        .filter(|x| x.codec_parameters().is_video())
        .next()
        .expect("Failed to find a video stream");

    let mut codec = CodecContext::from_decoder_id(video_stream.codec_parameters().codec_id())
        .expect("Failed to create CodecContext");

    codec.fill_from_parameters(&video_stream.codec_parameters());

    codec.open(None).unwrap();

    /*
    = CodecCtxBuilder::with_decoder(
        video_stream
            .codec_parameters()
            .codec_id())
    .fill_from_parameters(&video_stream.codec_parameters())
       .open(None)
       .unwrap();
       */

    format_context.seek_msec(video_stream.index(), 4 * 60 * 1000);

    let mut packet = Packet::new();

    while format_context.read_frame(&mut packet) >= 0 {
        println!("{}", packet.stream_index());

        if packet.stream_index() == video_stream.index() {
            codec.send_packet(&packet);

            let mut frame = Frame::empty().expect("Failed to allocate a frame");

            while codec.receive_frame(&mut frame) >= 0 {
                let (width, height) = video_stream.codec_parameters().size();
                println!("{width} x {height}");

                let mut new_frame =
                    Frame::from_size_and_pixfmt(width, height, AVPixelFormat_AV_PIX_FMT_RGB24)
                        .unwrap();

                let sws = Sws::new(
                    width,
                    height,
                    video_stream.codec_parameters().format(),
                    width,
                    height,
                    AVPixelFormat_AV_PIX_FMT_RGB24,
                    SWS_BILINEAR as _,
                );

                println!("{:?} {:?}", frame.linesize(), new_frame.linesize());

                sws.scale(&frame, &mut new_frame);

                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open("data.bin")
                    .unwrap();

                file.write(new_frame.data_plane(0).unwrap()).unwrap();
            }

            break;
        }
    }

    println!("Exit!");
}
