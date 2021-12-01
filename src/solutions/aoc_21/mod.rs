mod day1;
mod day2;

pub use day1::Day21_1;
pub use day2::Day21_2;

use std::default::Default;
use super::ThreadSafeSolvable;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref AOC21_SOLUTIONS: Vec<Box<ThreadSafeSolvable>> = {
        vec![
            Box::new(Day21_1::default()),
            Box::new(Day21_2::default()),
         ]
    };
}


