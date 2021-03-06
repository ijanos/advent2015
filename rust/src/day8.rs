use std::io;
use std::io::prelude::*;

enum State {
    Normal,
    Backslash,
    Hex(u8),
}

pub fn main() {
    let stdin = io::stdin();
    let mut chr = 0;
    let mut len = 0;
    let mut escaped = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        len += line.len();
        escaped += 2;
        let mut state = State::Normal;
        for c in line.chars() {
            if c == '"' || c == '\\' {
                escaped += 1
            };
            state = match state {
                State::Normal => {
                    chr += 1;
                    if c == '\\' {
                        State::Backslash
                    } else {
                        State::Normal
                    }
                }
                State::Backslash => {
                    if c == 'x' {
                        State::Hex(1)
                    } else {
                        State::Normal
                    }
                }
                State::Hex(x) => {
                    if x == 0 {
                        State::Normal
                    } else {
                        State::Hex(x - 1)
                    }
                }
            }
        }
        chr -= 2; // substract the open and close quotes
    }
    println!("Part 1: {}", len - chr);
    println!("Part 2: {}", escaped);
}
