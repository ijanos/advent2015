use std::io;
use std::io::prelude::*;

fn day1<R>(input: R) -> (i32, u32)
    where R: BufRead
{
    let mut part2 = None;
    let mut floor = 0;
    let mut n = 1;
    for line in input.lines() {
        for c in line.unwrap().chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!(),
            }
            if floor == -1 && part2 == None {
                part2 = Some(n);
            }
            n += 1;
        }
    }
    (floor, part2.unwrap())
}

pub fn main() {
    let stdin = io::stdin();
    let (part1, part2) = day1(stdin.lock());
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

#[test]
fn test() {
    use std::fs::File;
    use std::io::BufReader;
    let f = File::open("../puzzles/day01/input.txt").unwrap();
    let (part1, part2) = day1(BufReader::new(f));
    assert_eq!(part1, 138);
    assert_eq!(part2, 1771);
}
