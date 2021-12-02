use anyhow::{ Result, anyhow };
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 3>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_aoc21_3_p1() {
        assert!(true);
    }

    #[test]
    pub fn test_aoc21_3_p2() {
        assert!(true);
    }
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        Err(anyhow!("Not implemented"))
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        Err(anyhow!("Not implemented"))
    }
}
