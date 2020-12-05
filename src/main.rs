use std::env;

mod day1;
mod day2;

pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        day1::solve();
        day2::solve();
        return;
    }

    match args[1].as_str() {
        "day1" => day1::solve(),
        "day2" => day2::solve(),
        _ => println!("Incorrect day, example: day1")
    }
}
