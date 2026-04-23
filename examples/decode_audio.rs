use std::io::Write;

use libav_ng::{
    avcodec::{CodecContext, error::AVCodecError},
    avformat::FormatContext,
    avframe::Frame,
    avpacket::Packet,
};
use libav_sys_ng::{av_get_bytes_per_sample, av_sample_fmt_is_planar};

fn main() {
    let filename = match std::env::args().skip(1).next() {
        Some(f) => f,
        None => {
            eprintln!("Specify a file!");
            std::process::exit(1)
        }
    };

    let mut fmt = FormatContext::open_input(&filename).unwrap();

    let audio_stream = fmt
        .streams()
        .filter(|x| x.codec_parameters().is_audio())
        .next()
        .unwrap_or_else(|| panic!("No audio streams found!"));

    let mut decoder = CodecContext::from_decoder_id(audio_stream.codec_parameters().codec_id())
        .expect("Failed to find decoder!");

    decoder.fill_from_parameters(&audio_stream.codec_parameters());

    decoder.open(None).unwrap();
    
    let bps = unsafe { av_get_bytes_per_sample(decoder.sample_format()) };

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("output.pcm")
        .unwrap();

    let mut packet = Packet::new();

    while fmt.read_frame(&mut packet) >= 0 {
        if packet.stream_index() != audio_stream.index() {
            packet.clear();
            continue;
        }

        decoder.send_packet(&packet);

        let mut frame = Frame::empty().unwrap();

        loop {
            let code = decoder.receive_frame(&mut frame);

            match code {
                Ok(()) => (),
                Err(AVCodecError::TryAgain) => {
                    println!("EAGAIN!");
                    decoder.send_packet(&packet);
                    continue;
                }
                Err(AVCodecError::Eof) => {
                    break;
                }
                Err(other) => {
                    println!("Other error: {other:?}");
                }
            }

            // println!("Linesize: {:?}", frame.linesize());


            let is_planar = unsafe { av_sample_fmt_is_planar(decoder.sample_format()) } != 0;

            // println!("Planar: {is_planar}");

            if is_planar {
                for sample_idx in 0..frame.sample_count() {
                    for channel in 0..frame.channel_layout().nb_channels {
                        let data = &frame.data_plane(channel as usize).unwrap()[(sample_idx * bps) as usize..];

                        // output_file.write(&data[..bps as usize]).unwrap();
                    }
                }
            } else {
                let plane = frame.data_plane(0).unwrap();

                println!("Writing: {} ({:?})", plane.len(), frame.linesize());

                output_file.write(plane).unwrap();
            }

            // packet.clear();

            break;
        }
    }
}
