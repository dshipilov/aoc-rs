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

pub const YEAR: u32 = 2021;

use std::default::Default;
use super::ThreadSafeSolvable;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SOLUTIONS: Vec<Box<ThreadSafeSolvable>> = {
        vec![
            Box::new(day1::TheDay::default()),
            Box::new(day2::TheDay::default()),
            Box::new(day3::TheDay::default()),
            Box::new(day4::TheDay::default()),
            Box::new(day5::TheDay::default()),
            Box::new(day6::TheDay::default()),
            Box::new(day7::TheDay::default()),
            Box::new(day8::TheDay::default()),
            Box::new(day9::TheDay::default()),
            Box::new(day10::TheDay::default()),
            Box::new(day11::TheDay::default()),
         ]
    };
}


