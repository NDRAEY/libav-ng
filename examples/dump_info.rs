use libav_ng::avformat::FormatContext;

fn main() {
    let url = if let Some(url) = std::env::args().skip(1).next() {
        url
    } else {
        eprintln!("No input file!");
        std::process::exit(1);
    };

    let mut format_context = FormatContext::open_input(&url).expect("Failed to open file!");

    for i in format_context.streams() {
        let index = i.index();
        let parameters = i.codec_parameters();
        let name = parameters.codec_name();
        let duration_ms = (i.duration_sec() * 1000.0) as i64;

        let (hr, min, sec, msec) = {
            const HOUR: i64 = 3600 * 1000;

            (
                duration_ms / HOUR,
                (duration_ms % HOUR) / (60 * 1000),
                (duration_ms % (60 * 1000)) / 1000,
                duration_ms % 1000,
            )
        };

        let s_type = if parameters.is_audio() {
            "audio"
        } else if parameters.is_video() {
            "video"
        } else {
            "other"
        };

        println!("Stream #{index}: {s_type}, {name}, ({hr:02}:{min:02}:{sec:02}:{msec:03})");

        if parameters.is_video() {
            let (width, height) = parameters.size();
            let bps = parameters.bitrate();

            println!("       |- size: {width} x {height}; {bps} bps");
        } else if parameters.is_audio() {
            let bps = parameters.bitrate();

            println!("       |- {bps} bps");
        }
    }
}
