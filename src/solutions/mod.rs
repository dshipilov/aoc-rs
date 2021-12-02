use crate::common::{ Solvable, AocDate };
use lazy_static::lazy_static;
use std::collections::HashMap;

mod aoc_20;
mod aoc_21;

type ThreadSafeSolvable = dyn Solvable + Send + Sync;

lazy_static! {
    static ref ALL_SOLUTIONS: HashMap<u32, &'static Vec<Box<ThreadSafeSolvable>>> = {
        let mut m = HashMap::new();
        m.insert(aoc_20::YEAR, aoc_20::SOLUTIONS.as_ref());
        m.insert(aoc_21::YEAR, aoc_21::SOLUTIONS.as_ref());
        m
    };
}

pub fn find(date: &AocDate) -> Option<&Box<ThreadSafeSolvable>> {
    ALL_SOLUTIONS.get(&date.year).map(|&v| v.iter().find(|&e| e.get_date() == *date)).unwrap()
}

