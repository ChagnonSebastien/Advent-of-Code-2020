extern crate core;

mod parser;
mod day;
mod days;

use crate::days::get_day;

fn main() {
    let mut date = 1;
    let mut total_time = 0;

    loop {
        let day = get_day(date);
        if day.solutions.is_none() {
            break;
        }

        println!("=========================");
        println!("          Day {:02}         ", date);
        println!("=========================");
        total_time += day.benchmark(1);
        println!();

        date += 1;
    }

    println!("The {} days took a total time of {}μs", date - 1, total_time);
}
