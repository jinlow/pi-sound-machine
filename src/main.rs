mod whitenoise;

use rodio::{OutputStream, Sink, Source};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let sink = Sink::try_new(&stream_handle)?;

    // Make sure to add the `take_duration` call, otherwise this
    // will make an infinitely long vector and lead to a memory leak.
    let source = whitenoise::WhiteNoise::new(0.2)
        .take_duration(std::time::Duration::from_secs(30))
        .low_pass(500);

    // Here we repeat the sound infinitely.
    sink.append(source.repeat_infinite());
    sink.sleep_until_end();
    Ok(())
}
