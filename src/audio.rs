use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

pub fn play_morse(tokens: &[String]) {
    let mut _handle = DeviceSinkBuilder::open_default_sink().expect("Failed to open audio stream");
    _handle.log_on_drop(false); // silence the DeviceSink message

    let player = Player::connect_new(_handle.mixer());

    for letter in tokens {
        for token in letter.chars() {
            print!("{}", token);
            io::stdout().flush().unwrap();

            let duration_ms = if token == '.' { 80 } else { 240 };

            let tone = SineWave::new(600.0)
                .take_duration(Duration::from_millis(duration_ms))
                .amplify(0.20);

            player.append(tone);
            player.sleep_until_end();

            thread::sleep(Duration::from_millis(80));
        }

        print!(" ");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(240));
    }

    println!();
}
