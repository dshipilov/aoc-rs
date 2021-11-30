mod day1;
mod day2;

pub use day1::*;
pub use day2::*;

use crate::common::*;

pub fn all() -> Vec<Box<dyn Solvable>> {
    vec![
        Box::new(Day1::default()),
        Box::new(Day2::default()),
    ]
}


