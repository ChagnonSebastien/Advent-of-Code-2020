extern crate core;

mod array_utils;
mod parser;
mod day;
mod days;

use crate::days::get_day;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    benchmark: Option<usize>,

    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let (start, end) = match args.day {
        None => (1, 25),
        Some(day) => (day, day),
    };

    let mut total_time = 0;

    for date in start..end+1 {
        println!();
        let day = get_day(date);
        if day.solutions.is_none() {
            if args.day.is_some() {
                panic!("Day is not yet implemented")
            }
            break;
        }

        if args.day.is_none() {
            let width = 20;
            println!("{}Day {:02}{}", " ".repeat(width), date, " ".repeat(width));
            println!("{}", "═".repeat(width * 2 + 6));
        }
        total_time += day.run(args.benchmark);
        println!();

    }

    if args.benchmark.is_some() {
        println!("The total time for all the days ran was {} ms", total_time / 1000);
        println!()
    }
}
