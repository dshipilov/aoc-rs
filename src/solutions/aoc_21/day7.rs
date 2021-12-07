use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 7>;

fn fuel_func_p1(positions: &[i32], pos: i32) -> i32 {
    positions.iter().map(|p| i32::abs(*p - pos)).sum()
}

fn fuel_func_p2(positions: &[i32], pos: i32) -> i32 {
    positions.iter().map(|p| {
        let n = i32::abs(*p - pos);
        (n + 1) * n / 2 
    }).sum()
}

fn min_fuel(pos: &[i32], fuel_func: &mut dyn FnMut(&[i32], i32) -> i32) -> i32 {
    let n_pos = pos.iter().max().unwrap();

    let (_, min_fuel) = (0..*n_pos)
            .map(|p| (*fuel_func)(&pos, p))
            .enumerate()
            .min_by_key(|(_, fuel)| *fuel).unwrap();

    min_fuel
}

impl Solution for TheDay {
    type Output = i32;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let pos: Vec<i32> = input.split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();

        Ok(min_fuel(&pos, &mut fuel_func_p1))
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let pos: Vec<i32> = input.split(",").map(|s| s.trim().parse::<i32>().unwrap()).collect();

        Ok(min_fuel(&pos, &mut fuel_func_p2))
    }
}
