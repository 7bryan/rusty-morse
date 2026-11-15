# Product Requirements Document: Rust Morse Code Translator & Beeper (`rusty-morse`)

## 1. Overview

`rusty-morse` is a Command-Line Interface (CLI) tool written in Rust that converts human-readable text into International Morse Code, displays the visual representation in the terminal, and plays the corresponding audio tones out loud through the system's speakers.

## 2. Goals & Learning Objectives

- **Data Structures:** Use `HashMap` for character-to-Morse lookups.
- **Ownership & Strings:** Practice `String` vs `&str`, `.chars()`, and pattern matching (`match`).
- **Error/Option Handling:** Gracefully handle unsupported characters (e.g., emojis, special symbols) without panicking.
- **External Crates & Audio:** Integrate the `rodio` crate to generate continuous sine waves and manage playback timing with `std::thread::sleep`.

## 3. Functional Requirements

- **FR-1 (Input):** Accept a text string via CLI argument (e.g., `cargo run -- "SOS"`) or interactive standard input (`stdin`) if no argument is passed.
- **FR-2 (Translation):** Convert A-Z (case-insensitive), 0-9, and basic punctuation (`.`, `,`, `?`, `!`) into dots (`.`) and dashes (`-`).
- **FR-3 (Visual Output):** Print the translated Morse code to the console in real time synchronized with the audio, or print the full string before playback.
- **FR-4 (Audio Playback):** Play a clean 600 Hz sine wave tone for dots and dashes, separated by accurate Morse timing silences.
- **FR-5 (Graceful Fallback):** Ignore or mark unrecognized characters (e.g., print `?` or skip) rather than crashing.

## 4. Non-Functional Requirements

- **Zero-Panic Execution:** No `.unwrap()` on user input or audio device initialization where recoverable.
- **Cross-Platform:** Runs on Linux, macOS, and Windows (note: Linux requires ALSA dev libraries).
