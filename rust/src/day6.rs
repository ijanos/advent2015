use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum CommandType {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Command {
    command: CommandType,
    start: Point,
    end: Point
}

fn parse_coordinates(coordinate: &str) -> Point {
    let xy: Vec<usize> = coordinate.split(",")
                                   .map(|n| n.parse().unwrap())
                                   .collect();
    Point { x: xy[0], y: xy[1] }
}

fn get_cmd(input: &str) -> Command {
    let cmdline: Vec<&str> = input.split(" ").collect();
    let cmd = match cmdline[0] {
        "toggle" => CommandType::Toggle,
        "turn" => match cmdline[1] {
            "on" => CommandType::On,
            "off" => CommandType::Off,
            _ => panic!()
        },
        _ => panic!()
    };
    let (start, end) = match cmd {
        CommandType::Toggle => (parse_coordinates(cmdline[1]), parse_coordinates(cmdline[3])),
        _ => (parse_coordinates(cmdline[2]), parse_coordinates(cmdline[4])),

    };
    Command { command: cmd,
                start: start,
                  end: end
            }
}

fn part1_match(cmd: &Command, current: bool) -> bool {
    match cmd.command {
        CommandType::On => true,
        CommandType::Off => false,
        CommandType::Toggle => !current
    }
}

fn part2_match(cmd: &Command, current: u32) -> u32 {
    match cmd.command {
        CommandType::On => current + 1,
        CommandType::Off => if current < 2 { 0 } else { current - 1},
        CommandType::Toggle => current + 2
    }
}

pub fn main() {
    let mut grid1 = vec![vec![false; 1000]; 1000];
    let mut grid2 = vec![vec![0; 1000]; 1000];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let cmd = get_cmd(&line.unwrap());
        for x in cmd.start.x..cmd.end.x + 1 {
            for y in cmd.start.y..cmd.end.y + 1 {
                grid1[x][y] = part1_match(&cmd, grid1[x][y]);
                grid2[x][y] = part2_match(&cmd, grid2[x][y]);
            }
        }
    }
    println!("Part 1: {}", grid1.iter().flat_map(|row| row.iter()).filter(|&&v| v).count());
    println!("Part 2: {}", grid2.iter().flat_map(|row| row.iter()).fold(0, |acc, &l| acc + l));
}
