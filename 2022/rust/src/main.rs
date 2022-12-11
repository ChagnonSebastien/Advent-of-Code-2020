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
    let mut amount_days_ran = 0;

    for date in start..end+1 {
        let day = get_day(date);
        if day.solutions.is_none() {
            break;
        }

        println!("=========================");
        println!("          Day {:02}         ", date);
        println!("=========================");
        total_time += day.run(args.benchmark);
        amount_days_ran += 1;
        println!();

    }

    if args.benchmark.is_some() {
        println!("The total time of all the days ran was {}μs", total_time);
    }
}
