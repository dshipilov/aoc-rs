use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 1>;

fn str_to_ints_vec(str: &str) -> Vec<i32> {
    str.lines().into_iter()
    .map(|s| s.parse::<i32>().unwrap())
    .collect()
}

fn count_increasing(data: &[i32]) -> usize {
    data.iter().zip(data.iter().skip(1)).filter(|(a,b)| a < b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_int_lines() {
        let input = "123\n247\n5\n24";
        let v = vec![123, 247, 5, 24];
        assert_eq!(v, str_to_ints_vec(input));
    }


    #[test]
    pub fn test_day21_1_p1_solution() {
        let data: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let count = count_increasing(&data);
        assert_eq!(count, 7);
    }

    #[test]
    pub fn test_day21_1_p2_solution() {
        let data: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let data_by_3_sum: Vec<i32> = data.windows(3).map(|w| w.iter().sum()).collect();
        assert_eq!(data_by_3_sum, vec![607, 618, 618, 617, 647, 716, 769, 792]);

        let count = count_increasing(&data_by_3_sum);
        assert_eq!(count, 5);
    }
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let data: Vec<i32> = str_to_ints_vec(input);

        Ok(count_increasing(&data))
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let data: Vec<i32> = 
            str_to_ints_vec(input)
            .windows(3)
            .map(|w| w.iter().sum())
            .collect();

        Ok(count_increasing(&data))
    }
}
