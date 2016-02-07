use std::collections::HashSet;

use std::io;
use std::io::prelude::*;


pub fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut rx = 0;
    let mut ry = 0;
    let mut houses = HashSet::new();
    houses.insert((x,y));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let line = line.unwrap();
      for c in line.chars() {
            match c.to_string().as_ref() {
                ">" => x += 1,
                "<" => x -= 1,
                "^" => y += 1,
                "v" => y -= 1,
                _ => panic!()
            }
            houses.insert((x,y));
      }
    }
    println!("Part 1: {}", houses.len());
}