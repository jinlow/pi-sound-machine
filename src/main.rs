use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("resources/brown-noise-seamless.mp3")
    .expect("Unable to read file."));
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
