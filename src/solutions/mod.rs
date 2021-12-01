mod day21_1;
mod day21_2;


pub use day21_1::*;
pub use day21_2::*;

use std::default::Default;
use crate::common::*;

use lazy_static::lazy_static;
type ThreadSafeSolvable = dyn Solvable + Send + Sync;

lazy_static! {
    static ref ALL_SOLUTIONS: Vec<Box<ThreadSafeSolvable>> = {
        vec![
            Box::new(Day21_1::default()),
            Box::new(Day21_2::default()),
         ]
    };
}

pub fn find(date: &AocDate) -> Option<&Box<ThreadSafeSolvable>> {
    ALL_SOLUTIONS.iter().find(|&e| e.get_date() == *date)
}

pub fn all() -> Vec<Box<dyn Solvable>> {
    vec![
        Box::new(Day21_1::default()),
        Box::new(Day21_2::default()),
    ]
}


