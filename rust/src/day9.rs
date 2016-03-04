use std::{io, u32};
use std::io::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use permutohedron::Heap as Permutations;

pub fn main() {
    let stdin = io::stdin();
    let mut distances = HashMap::<(String, String), u32>::new();
    let mut cities = HashSet::<String>::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<_>>();
        let (city1, city2, distance) = (line[0], line[2], line[4].parse::<u32>().unwrap());
        cities.insert(city1.to_owned());
        cities.insert(city2.to_owned());
        distances.insert((city1.to_owned(), city2.to_owned()), distance);
        distances.insert((city2.to_owned(), city1.to_owned()), distance);
    }
    let mut cities = cities.iter().collect::<Vec<_>>();
    let permutations = Permutations::new(&mut cities);
    let mut longest = 0;
    let mut shortest = u32::MAX;
    for cityorder in permutations {
        let mut path = 0;
        for (&city1, &city2) in cityorder.iter().zip(&cityorder[1..]) {
            if let Some(&distance) = distances.get(&(city1.to_owned(), city2.to_owned())) {
                path += distance;
            }
        }
        longest = max(path, longest);
        shortest = min(path, shortest)
    }
    println!("Part 1: {}", shortest);
    println!("Part 2: {}", longest);
}
