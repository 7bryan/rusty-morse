use std::env;
mod audio;
mod morse;

fn main() {
    let args: Vec<String> = env::args().collect();

    let messages = if args.len() > 1 {
        &args[1]
    } else {
        println!("Please provided a valid args\nUsage: cargo run -- <morse-code>");
        return;
    };

    let morse_token: Vec<String> = morse::translate(&messages);
    // println!("{}", morse_token.join(" "));

    println!();

    audio::play_audio();
}
