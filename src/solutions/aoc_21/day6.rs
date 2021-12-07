use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 6>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_aoc21_6() {
        let mut population: Vec<u8> = "3,4,3,1,2".split(",").map(|s| s.parse::<u8>().unwrap()).collect();

        for _ in 0..80 {
            let mut to_extend: usize = 0;

            for v in &mut population {
                if *v == 0 {
                    *v = 6;
                    to_extend += 1;
                } else {
                    *v -= 1;
                }
            }

            for _ in 0..to_extend {
                population.push(8);
            }
        }

        let mut agg = Aggregate::from_string("3,4,3,1,2");
 
        assert_eq!(population.len(), agg.simulate(80) as usize);
    }
}

struct Aggregate {
    life_times: [u64; 9]
}

impl Aggregate {
    fn from_string(seed: &str) -> Self {
        let mut life_times = [0u64; 9];

        seed.split(",")
            .map(|s| { s.trim().parse::<usize>().unwrap() })
            .for_each(|n| life_times[n] += 1);

        Aggregate { life_times }
    }

    fn simulate(&mut self, n_days: usize) -> u64 {
        for iter in 1..n_days {
            let day_from = iter % 9;
            let day_to = (iter + 7) % 9;

            self.life_times[day_to] += self.life_times[day_from];
        }

        self.life_times.iter().sum()
    }
}

impl Solution for TheDay {
    type Output = u64;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let mut agg = Aggregate::from_string(input);
        Ok(agg.simulate(80))
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let mut agg = Aggregate::from_string(input);
        Ok(agg.simulate(256))
    }
}
