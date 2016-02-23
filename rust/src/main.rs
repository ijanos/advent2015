extern crate crypto;
extern crate itertools;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
            _ => println!("No such day")
        }
    }

}
