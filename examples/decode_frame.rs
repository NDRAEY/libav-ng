use std::io::Write;

use libav_ng::{
    avcodec::error::AVCodecError, avformat::FormatContext, avframe::Frame, avpacket::Packet, sws::Sws,
};
use libav_sys_ng::{
    AVPixelFormat_AV_PIX_FMT_RGB24, SwsFlags_SWS_BILINEAR
};

fn main() {
    let url = if let Some(url) = std::env::args().skip(1).next() {
        url
    } else {
        eprintln!("No input file!");
        std::process::exit(1);
    };

    let time = if let Some(time) = std::env::args().skip(2).next() {
        time.parse::<usize>().expect("Failed to parse time in milliseconds")
    } else {
        0
    };

    let mut format_context = FormatContext::open_input(&url).expect("Failed to open file!");

    for i in format_context.streams() {
        println!("{i:#?}");
    }

    let video_stream = format_context
        .streams()
        .filter(|x| x.codec_parameters().is_video())
        .next()
        .expect("Failed to find a video stream");

    let mut codec = video_stream.codec_parameters().to_decoder()
        .expect("Failed to create CodecContext");

    // let mut codec = CodecContext::from_decoder_id(video_stream.codec_parameters().codec_id())
    //     .expect("Failed to create CodecContext");

    // codec.fill_from_parameters(&video_stream.codec_parameters());

    codec.open(None).unwrap();

    format_context.seek_msec(video_stream.index(), time as i64);

    let mut packet = Packet::new();

    while format_context.read_frame(&mut packet) >= 0 {
        println!("Stream index: {}", packet.stream_index());

        if packet.stream_index() == video_stream.index() {
            codec.send_packet(&packet);
            println!("Sent!");

            let mut frame = Frame::empty().expect("Failed to allocate a frame");

            loop {
                let code = codec.receive_frame(&mut frame);

                match code {
                    Ok(()) => (),
                    Err(AVCodecError::TryAgain) => {
                        codec.send_packet(&packet);
                        continue;
                    }
                    Err(err) => {
                        panic!("Other error: {err:?}");
                    }
                }
                
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
                    SwsFlags_SWS_BILINEAR as _,
                );

                println!("{:?} {:?}", frame.linesize(), new_frame.linesize());

                sws.scale(&frame, &mut new_frame);

                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open("data.bin")
                    .unwrap();

                println!("Len: {:?}", new_frame.data_plane(0).unwrap().len());

                file.write(new_frame.data_plane(0).unwrap()).unwrap();

                break;
            }

            break;
        }
    }

    println!("Exit!");
}
