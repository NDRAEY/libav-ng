use std::io::Write;

use libav_ng::{avcodec::CodecContext, avformat::FormatContext, avframe::Frame, avpacket::Packet};
use libav_sys_ng::{AVPixelFormat_AV_PIX_FMT_RGB24, SWS_BILINEAR, SwsContext, sws_freeContext, sws_getContext, sws_scale};

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

    println!("CodecParams idx: {}", video_stream.index());
    println!("CodecParams format: {}", video_stream.codec_parameters().format());

    let mut codec = CodecContext::from_decoder_id(video_stream.codec_parameters().codec_id())
        .expect("Failed to create CodecContext");

    codec.fill_from_parameters(&video_stream.codec_parameters());

    codec.open(None).unwrap();

    println!("Pixel format: {}", codec.get_pixel_format());

    format_context.seek_msec(video_stream.index(), 5 * 60 * 1000);

    let mut packet = Packet::new();

    while format_context.read_frame(&mut packet) >= 0 {
        println!("{}", packet.stream_index());

        if packet.stream_index() == video_stream.index() {
            println!(
                "{} # {} _ {}",
                packet.position(),
                packet.duration(),
                packet.data().len()
            );

            codec.send_packet(&packet);

            let mut frame = Frame::empty().expect("Failed to allocate a frame");

            while codec.receive_frame(&mut frame) >= 0 {                
                println!("{}", frame.format());
                {
                    unsafe {
                        let (width, height) = video_stream.codec_parameters().size();
                        println!("{width} x {height}");

                        let mut new_frame = Frame::from_size_and_pixfmt(width, height, AVPixelFormat_AV_PIX_FMT_RGB24).unwrap();
                        new_frame.allocate_buffer();

                        let swsContext: *mut SwsContext = sws_getContext(
                            width, height,
                            video_stream.codec_parameters().format(),
                            width, height,
                            AVPixelFormat_AV_PIX_FMT_RGB24,
                            SWS_BILINEAR as _, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null()
                        );

                        println!("{:?} {:?}", frame.linesize(), new_frame.linesize());

                        sws_scale(
                            swsContext,
                            (*frame.raw()).data.as_ptr().cast(),
                            frame.linesize().as_ptr(),
                            0,
                            height,
                            (*new_frame.raw_mut()).data.as_ptr(), new_frame.linesize().as_ptr()
                        );


                        let mut file = std::fs::OpenOptions::new()
                            .create(true)
                            .truncate(true)
                            .write(true)
                            .open("data.bin")
                            .unwrap();

                        file.write(new_frame.data_plane(0).unwrap()).unwrap();

                        sws_freeContext(swsContext);
                    }
                }
            }

            break;
        }
    }

    println!("Exit!");
}
