extern crate lazy_static;
extern crate log;

mod common;
mod logging;
mod solutions;

use log::{ LevelFilter, info };
use common::AocDate;

fn main() {
    logging::setup(LevelFilter::Info);

    let args: Vec<String> = std::env::args().collect();

    let date = if args.len() > 2 {
        let year = args[1].parse::<u32>().unwrap();
        let day = args[2].parse::<u32>().unwrap();

        AocDate { year, day } 
    } else if args.len() > 1 {
        let day = args[1].parse::<i32>().unwrap();
        AocDate::day(day)
    } else {
        AocDate::today()
    };

    info!("Solving {}", date);
 
    solutions::find(&date).unwrap().solve();
}

