use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn part1() -> i32 {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
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

fn part2() {}

pub fn main() {
    println!("part 1: {}", part1());
}