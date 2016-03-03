use std::io;
use std::io::prelude::*;

fn day2<R> (input: R) -> (i32, i32)
    where R: BufRead
{
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let split = line.split("x");
        let mut nums = split.map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        nums.sort();
        let area1 = nums[0] * nums[1];
        let area2 = nums[0] * nums[2];
        let area3 = nums[1] * nums[2];
        part1 += area1 * 2 + area2 * 2 + area3 * 2 + area1;
        part2 += nums[0] * 2 + nums[1] * 2 + nums[0] * nums[1] * nums[2];
    }
    (part1, part2)
}

pub fn main() {
    let stdin = io::stdin();
    let (part1, part2) = day2(stdin.lock());
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

#[test]
fn test() {
    use std::fs::File;
    use std::io::BufReader;
    let f = File::open("../puzzles/day02/input.txt").unwrap();
    let (part1, part2) = day2(BufReader::new(f));
    assert_eq!(part1, 1598415);
    assert_eq!(part2, 3812909);
}
