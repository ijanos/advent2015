use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

struct Santa {
    x: i32,
    y: i32
}

fn step(c: char, s: &mut Santa) {
    match c {
        '>' => s.x += 1,
        '<' => s.x -= 1,
        '^' => s.y += 1,
        'v' => s.y -= 1,
        _ => panic!()
    }
}

pub fn main() {
    let mut p1_santa = Santa {x: 0, y: 0};
    let mut p2_santa = Santa {x: 0, y: 0};
    let mut p2_robo  = Santa {x: 0, y: 0};

    let mut part1_visited = HashSet::new();
    let mut part2_visited = HashSet::new();

    part1_visited.insert((0,0));
    part2_visited.insert((0,0));

    let mut roboturn: bool = false;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let line = line.unwrap();
      for c in line.chars() {
            step(c, &mut p1_santa);
            if roboturn {
                step(c, &mut p2_robo);
            } else {
                step(c, &mut p2_santa);
            }
            roboturn = !roboturn;
            part1_visited.insert((p1_santa.x, p1_santa.y));
            part2_visited.insert((p2_santa.x, p2_santa.y));
            part2_visited.insert((p2_robo.x, p2_robo.y));
      }
    }
    println!("Part 1: {}", part1_visited.len());
    println!("Part 2: {}", part2_visited.len());
}