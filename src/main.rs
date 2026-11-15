use std::env;
mod morse;

fn main() {
    let args: Vec<String> = env::args().collect();

    let messages = if args.len() > 1 {
        &args[1]
    } else {
        println!("Please provided a valid args\nUsage: cargo run -- <morse-code>");
        return;
    };

    for msg in messages.chars() {
        if let Some(code) = morse::MORSE_CODE.get(&msg.to_ascii_uppercase()) {
            print!("{} ", code);
        }
    }

    println!();

    // println!("{}", morse::MORSE_CODE.get(&'H').unwrap());

    // if let Some(code) = morse::MORSE_CODE.get(&'H') {
    //     println!("{}", code);
    // }
}
