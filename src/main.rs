use std::env;
mod audio;
mod morse;

fn main() {
    let args: Vec<String> = env::args().collect();

    let messages = if args.len() > 1 {
        &args[1..].join(" ")
    } else {
        println!("Please provided a valid args\nUsage: cargo run -- <morse-code>");
        return;
    };

    let morse_token = morse::translate(messages);

    audio::play_morse(&morse_token);
}
