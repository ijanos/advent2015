mod day1;
mod day2;
mod day3;

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
            _ => println!("No such day")
        }
    }

}
