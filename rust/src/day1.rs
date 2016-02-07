use std::io;
use std::io::prelude::*;

pub fn main() {
    let stdin = io::stdin();
    let mut floor = 0;
    let mut part2 = None;
    let mut n = 1;
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!()
            }
            if floor == -1 && part2 == None {
                part2 = Some(n);
            }
            n += 1;
        }
    }
    println!("part 1: {}", floor);
    println!("part 2: {}", part2.unwrap());
}