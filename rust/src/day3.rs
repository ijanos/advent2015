use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::mem;

struct Santa {
    x: i32,
    y: i32,
}

struct Day03 {
    part1: Santa,
    part2: Santa,
    temp: Santa,
}

impl Santa {
    fn step(&mut self, c: char) {
        match c {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _ => panic!(),
        }
    }
}

pub fn main() {
    let mut day03 = Day03 {
        part1: Santa { x: 0, y: 0 },
        part2: Santa { x: 0, y: 0 },
        temp: Santa { x: 0, y: 0 },
    };

    let mut part1_visited = HashSet::new();
    let mut part2_visited = HashSet::new();

    part1_visited.insert((0, 0));
    part2_visited.insert((0, 0));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for c in line.chars() {
            day03.part1.step(c);
            day03.part2.step(c);
            part1_visited.insert((day03.part1.x, day03.part1.y));
            part2_visited.insert((day03.part2.x, day03.part2.y));
            mem::swap(&mut day03.part2, &mut day03.temp)
        }
    }
    println!("Part 1: {}", part1_visited.len());
    println!("Part 2: {}", part2_visited.len());
}
