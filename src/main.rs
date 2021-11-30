extern crate log;

use log::{ LevelFilter, info };
use common::AocDate;

mod common;
mod logging;
mod solutions;

const YEAR: u32 = 2020;

fn main() {
    logging::setup(LevelFilter::Debug);

    let mut all_solutions = solutions::all();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let day = args[1].parse::<usize>().unwrap();

        info!("Solving day {}", day);
        
        let solution = all_solutions[day - 1].as_mut();
        let (p1, p2) = solution.solve();

        if let Some(result) = p1 {
            info!("Part 1 solution: {}", result);
        } else {
            info!("No soluton for part 1");
        }

        if let Some(result) = p2 {
            info!("Part 2 solution: {}", result);
        } else {
            info!("No soluton for part 2");
        }
    } else {
        let _day = AocDate::today();

        //let mut solution = solutions::find_by_date(day);
    }
}

