extern crate crypto;
extern crate itertools;
extern crate permutohedron;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide a day")
    } else {
        match args[1].as_ref() {
            "1" => day1::main(),
            "2" => day2::main(),
            "3" => day3::main(),
            "4" => day4::main(),
            "5" => day5::main(),
            "6" => day6::main(),
            "7" => day7::main(),
            "8" => day8::main(),
            "9" => day9::main(),
            "10" => day10::main(),
            "11" => day11::main(),
            _ => println!("No such day"),
        }
    }

}
