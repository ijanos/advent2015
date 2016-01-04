use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn part1() -> i32 {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let mut floor:i32 = 0;
    for line in file.lines() {
        for c in line.unwrap().chars() {
            match c.to_string().as_ref() {
                "(" => floor += 1,
                ")" => floor -= 1,
                _ => panic!()
            }
        }
    }
    floor
}

fn part2() -> Option<i32> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let mut floor :i32 = 0;
    let mut n :i32 = 1;
    for line in file.lines() {
        for c in line.unwrap().chars() {
            match c.to_string().as_ref() {
                "(" => floor += 1,
                ")" => floor -= 1,
                _ => panic!()
            }
            if floor == -1 {
                return Some(n);
            }
            n += 1;
        }
    }
    None
}

pub fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2().unwrap());
}