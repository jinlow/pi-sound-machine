mod whitenoise;

use rodio::{OutputStream, Sink, Source};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = whitenoise::WhiteNoise::new(0.2)
        .take_duration(std::time::Duration::from_secs(30))
        .low_pass(500);

    // We want the sound to go forever, however, I need to figure
    // out how to make the fade smoother.
    sink.append(source.repeat_infinite());
    sink.sleep_until_end();
    Ok(())
}
