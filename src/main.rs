mod whitenoise;

use rodio::{Decoder, OutputStream, Sink, Source, source::SineWave};
use std::time::Duration;
use std::error::Error;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create an array of all the sounds we want included
    // in the sound machine, these will be precompilfed into the
    // actual binary, to make it more portable.
    //let mut sounds: HashMap<String, &[u8;0]> = HashMap::new();
    let sounds: [&[u8]; 4] = [
        std::include_bytes!("../resources/gentle-white-noise.mp3"),
        std::include_bytes!("../resources/brown-noise-seamless.mp3"),
        std::include_bytes!("../resources/brown-noise-seamless-short.mp3"),
        std::include_bytes!("../resources/white-noise-short.mp3"),
    ];
    let sink = Sink::try_new(&stream_handle).unwrap();
    let cursor = Cursor::new(sounds[2]);
    // let source = Decoder::new(cursor).unwrap();
    // let source = SineWave::new(100.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
    let source = whitenoise::WhiteNoise::new(0.8); //.amplify(100.0);
    // We want the sound to go forever, however, I need to figure
    // out how to make the fade smoother.
    sink.append(source.repeat_infinite());
    sink.sleep_until_end();
    Ok(())
}
