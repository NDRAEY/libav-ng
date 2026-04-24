use std::io::Write;

use libav_ng::{
    avcodec::{error::AVCodecError},
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
        .expect("No audio streams found!");


    let mut decoder = audio_stream.codec_parameters().to_decoder().expect("Failed to find decoder!");

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
                let mut buffer: Vec<u8> = Vec::with_capacity((frame.sample_count() * frame.channel_layout().nb_channels) as _);

                for sample_idx in 0..frame.sample_count() {
                    for channel in 0..frame.channel_layout().nb_channels {
                        let data = &frame.data_plane(channel as usize).unwrap()
                            [(sample_idx * bps) as usize..][..bps as usize];

                        // We must buffer our writes, because all blocks have small size.
                        buffer.extend_from_slice(data);
                    }
                }

                let current_pts = frame.pts();
                let total_duration = audio_stream.duration();

                print!("\r{} / {} ({:.2}%)", current_pts, total_duration, (current_pts as f64 / total_duration as f64) * 100.0);

                output_file.write(&buffer).unwrap();
            } else {
                let plane = frame.data_plane(0).unwrap();

                output_file.write(plane).unwrap();
            }

            // packet.clear();

            break;
        }
    }
}
