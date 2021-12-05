use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 3>;

fn bin_vec_to_num(v: &[u32]) -> u32 {
    let msb_idx = v.len() - 1;
    v.iter().enumerate().fold(0u32, |acc, (i, b)| acc | (b << msb_idx - i))
}

fn str_to_bin_vec(s: &str) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(2).unwrap()).collect()
}

fn load_popcount(input: &str) -> (u32, Vec<u32>) {
    let mut lines = input.lines();
    if let Some(init_str) = lines.next() {
        let mut total_len = 1;
        let mut bin_vec = str_to_bin_vec(init_str);

        for s in lines {
            for (i, b) in s.chars().map(|c| c.to_digit(2).unwrap()).enumerate() {
                bin_vec[i] += b
            }

            total_len += 1;
        }

        (total_len, bin_vec)
    } else {
        panic!("Unexpected empty input");
    }
}

// TODO: replace with reduce
fn popcount(bit: usize, v: &[u32]) -> u32 {
    let mut popcount = 0u32;
    for item in v {
        popcount += (item >> bit) & 1;
    }

    popcount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_aoc21_3_p1() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let (n, pop_counts) = load_popcount(input);
        let threshold = n / 2;

        let most_common_bits: Vec<u32> = pop_counts.iter().map(|n_ones| if n_ones > &threshold { 1u32 } else { 0u32 }).collect();

        let gamma = bin_vec_to_num(&most_common_bits);
        let epsilon = !gamma & ((1 << pop_counts.len()) - 1);

        assert_eq!(most_common_bits, vec![1, 0, 1, 1, 0]);
        assert_eq!(22, bin_vec_to_num(&most_common_bits));
        assert_eq!(9, epsilon);
    }

    #[test]
    pub fn test_aoc21_3_p2() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert!(true);
    }
}

impl Solution for TheDay {
    type Output = u32;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let (n, pop_counts) = load_popcount(input);
        let threshold = n / 2;

        let most_common_bits: Vec<u32> = pop_counts.iter().map(|n_ones| if n_ones > &threshold { 1 } else { 0 }).collect();

        let gamma = bin_vec_to_num(&most_common_bits);
        let epsilon = !gamma & ((1 << pop_counts.len()) - 1);

        Ok(gamma * epsilon)   
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let n_bits = 12;
        let mut oxygen_nums: Vec<u32> = input.lines().map(|s| u32::from_str_radix(s, 2).unwrap()).collect();
        let mut co2_nums: Vec<u32> = input.lines().map(|s| u32::from_str_radix(s, 2).unwrap()).collect();
 
        let mut oxygen_val = 0u32;
        let mut co2_val = 0u32;

        for shift in (0..n_bits).rev() {
            let n_ones = popcount(shift, &oxygen_nums);
            let n_zeros = oxygen_nums.len() as u32 - n_ones;
            let bit = if n_ones >= n_zeros{ 1 } else { 0 };

            oxygen_nums.retain(|v| (v >> shift) & 1 == bit);

            if oxygen_nums.len() == 1 {
                oxygen_val = oxygen_nums[0];
                break;
            }
        }

        for shift in (0..n_bits).rev() {
            let n_ones = popcount(shift, &co2_nums);
            let n_zeros = co2_nums.len() as u32 - n_ones;
            let bit = if n_zeros > n_ones { 1 } else { 0 };

            co2_nums.retain(|v| (v >> shift) & 1 == bit);

            if co2_nums.len() == 1 {
                co2_val = co2_nums[0];
                break;
            }
        }

        Ok(oxygen_val * co2_val)
    }
}
