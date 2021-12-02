mod day1;

pub const YEAR: u32 = 2020;

use std::default::Default;
use super::ThreadSafeSolvable;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SOLUTIONS: Vec<Box<ThreadSafeSolvable>> = {
        vec![
            Box::new(day1::TheDay::default()),
        ]
    };
}


