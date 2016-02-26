use std::io;
use std::io::prelude::*;
use itertools::Zip;
use std::collections::HashMap;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BAD_STRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

fn part1_nice(input: &str) -> bool {
    if BAD_STRINGS.iter().any(|&s| input.contains(s)) {
        return false;
    }
    if input.chars().filter(|c| VOWELS.contains(c)).count() < 3 {
        return false;
    }
    if !input.chars().zip(input[1..].chars()).any(|(c1, c2)| c1 == c2) {
        return false;
    }
    true
}

fn part2_nice(input: &str) -> bool {
    if !Zip::new((input.chars(), input[1..].chars(), input[2..].chars())).any(|(a, _, b)| a == b) {
        return false;
    };
    let mut doubles: HashMap<(char, char), usize> = HashMap::new();
    for (i, (c1, c2)) in Zip::new((input.chars(), input[1..].chars())).enumerate() {
        let j = doubles.entry((c1, c2)).or_insert(i);
        if i > *j + 1 {
            return true;
        }
    }
    false
}

pub fn main() {
    let stdin = io::stdin();
    let (result1, result2) = stdin.lock()
                                  .lines()
                                  .map(|line| line.unwrap())
                                  .fold((0, 0), |(part1, part2), line| {
                                      let part1 = if part1_nice(&line) {
                                          part1 + 1
                                      } else {
                                          part1
                                      };
                                      let part2 = if part2_nice(&line) {
                                          part2 + 1
                                      } else {
                                          part2
                                      };
                                      (part1, part2)
                                  });
    println!("Part 1: {}\nPart 2: {}", result1, result2);
}
