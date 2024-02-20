use rodio::{OutputStream, Source};
use std::time::Duration;

pub fn play_tone(frequency: f32, duration: Duration) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = rodio::source::SineWave::new(frequency);
    stream_handle.play_raw(source.convert_samples()).unwrap();
}


