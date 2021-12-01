extern crate lazy_static;
extern crate log;

mod common;
mod logging;
mod solutions;

use std::borrow::BorrowMut;

use log::{ LevelFilter, info };
use common::AocDate;

fn main() {
    logging::setup(LevelFilter::Debug);

    let mut all_solutions = solutions::all();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let day = args[1].parse::<usize>().unwrap();

        info!("Solving {}", day);
        
        let solution = all_solutions[day - 1].as_mut();
        solution.solve();
    } else {
        let day = AocDate::today();

        let solution = solutions::find(&day).unwrap().borrow_mut().solve();
        //solution.solve();
        //let mut solution = solutions::find_by_date(day);
    }
}

