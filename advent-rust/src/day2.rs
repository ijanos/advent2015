use std::io;
use std::io::prelude::*;

pub fn main() {
    let stdin = io::stdin();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in stdin.lock().lines() {
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
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}