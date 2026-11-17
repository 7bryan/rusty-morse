use std::collections::HashMap;
use std::sync::LazyLock;

pub static MORSE_CODE: LazyLock<HashMap<char, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ('A', ".-"),
        ('B', "-..."),
        ('C', "-.-."),
        ('D', "-.."),
        ('E', "."),
        ('F', "..-."),
        ('G', "--."),
        ('H', "...."),
        ('I', ".."),
        ('J', ".---"),
        ('K', "-.-"),
        ('L', ".-.."),
        ('M', "--"),
        ('N', "-."),
        ('O', "---"),
        ('P', ".--."),
        ('Q', "--.-"),
        ('R', ".-."),
        ('S', "..."),
        ('T', "-"),
        ('U', "..-"),
        ('V', "...-"),
        ('W', ".--"),
        ('X', "-..-"),
        ('Y', "-.--"),
        ('Z', "--.."),
        ('0', "-----"),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
    ])
});

pub fn translate(input: &str) -> Vec<(char, &'static str)> {
    let mut morse_token: Vec<(char, &'static str)> = Vec::new();
    for ch in input.chars() {
        let upper = ch.to_ascii_uppercase();
        if ch == ' ' {
            morse_token.push((' ', "/"));
        } else if let Some(code) = MORSE_CODE.get(&upper) {
            morse_token.push((upper, code));
        }
    }

    morse_token
}
