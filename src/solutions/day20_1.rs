use anyhow::{ Result, anyhow };
use crate::common::{ Solution, Day };
use log::{ info, error };

pub type Day1 = Day<2020, 1>;

const SUM: u32 = 2020;

fn str_to_ints_vec(str: &str) -> Vec<i32> {
    str.lines().into_iter()
    .map(|s| s.parse::<i32>().unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day20_1::*;

    #[test]
    pub fn test_parse_int_lines() {
        let input = "123\n247\n5\n24";
        let v = vec![123, 247, 5, 24];
        assert_eq!(v, str_to_ints_vec(input));
    }
}

impl Solution for Day<2020, 1> {
    type Output = i32;

    fn part1(&mut self, input: &str) -> Result<Self::Output> {
        let data = str_to_ints_vec(input);    

        Err(anyhow!("Not implemented"))
    }

    fn part2(&mut self) -> Result<Self::Output> {
        Err(anyhow!("Not implemented"))
    }
}
