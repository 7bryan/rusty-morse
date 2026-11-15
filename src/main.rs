mod morse;

fn main() {
    println!("{}", morse::MORSE_CODE.get(&'H').unwrap());

    if let Some(code) = morse::MORSE_CODE.get(&'H') {
        println!("{}", code);
    }
}
