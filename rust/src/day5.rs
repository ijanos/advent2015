use std::io;
use std::io::prelude::*;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BAD_STRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

fn nice(input: &str) -> bool {
    if BAD_STRINGS.iter().any(|&s| input.contains(s)) {
        return false;
    }
    if input.chars().filter(|c| VOWELS.contains(c)).count() < 3 {
        return false;
    }
    if !input.chars().zip(input[1..].chars()).any(|(c1,c2)| c1 == c2) {
        return false;
    }
    true
}

pub fn main() {
    let stdin = io::stdin();
    println!("{}", stdin.lock()
                        .lines()
                        .map(|line| line.unwrap())
                        .filter(|line| nice(&line))
                        .count());
}
