use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::time::Duration;

pub fn play_audio() {
    let _handle = DeviceSinkBuilder::open_default_sink().expect("Failed to open audio stream");
    // _handle.log_on_drop(false); // to silence the DeviceSink message

    let player = Player::connect_new(_handle.mixer());

    let tone = SineWave::new(600.0)
        .take_duration(Duration::from_millis(200))
        .amplify(0.20);

    player.append(tone);
    player.sleep_until_end();
}
