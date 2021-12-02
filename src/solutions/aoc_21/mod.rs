mod day1;
mod day2;

pub const YEAR: u32 = 2021;

use std::default::Default;
use super::ThreadSafeSolvable;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SOLUTIONS: Vec<Box<ThreadSafeSolvable>> = {
        vec![
            Box::new(day1::TheDay::default()),
            Box::new(day2::TheDay::default()),
         ]
    };
}


