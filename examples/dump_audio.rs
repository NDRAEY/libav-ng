use libav_ng::avformat::FormatContext;

fn main() {
    let filename = match std::env::args().skip(1).next() {
        Some(f) => f,
        None => {
            eprintln!("Specify a file!");
            std::process::exit(1)
        }
    };

    let mut fmt = FormatContext::open_input(&filename);

    todo!("Rest of the implementation");
}
