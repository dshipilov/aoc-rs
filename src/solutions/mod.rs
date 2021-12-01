mod aoc_21;

use crate::solutions::aoc_21::AOC21_SOLUTIONS;

use crate::common::{ Solvable, AocDate };

type ThreadSafeSolvable = dyn Solvable + Send + Sync;

fn merge_all() -> Vec<Box<ThreadSafeSolvable>> {
    let mut v: Vec<Box<ThreadSafeSolvable>> = Vec::new();
    v.extend(AOC21_SOLUTIONS.as_ref().iter());
    v
}

pub fn find(date: &AocDate) -> Option<&Box<ThreadSafeSolvable>> {
    merge_all().iter().find(|&e| e.get_date() == *date)
}

