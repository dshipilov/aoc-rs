use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

use regex::Regex;
use std::collections::HashSet;

pub type TheDay = Day<YEAR, 4>;

const SIZE: usize = 5;
type Slice = [u8; SIZE];

struct Board {
    slices: [Slice; SIZE*2],
}

impl Board {
    fn from_string(str_board: &[&str]) -> Self {
        let mut slices = [[0u8; SIZE]; SIZE*2];
        let split_regex = Regex::new(r"\s+").expect("Invalid regex syntax"); 

        for row in 0..SIZE {
            let str_row: Vec<u8> = split_regex.split(str_board[row].trim()).into_iter().map(|v| v.parse::<u8>().unwrap()).collect();
            for col in 0..SIZE {
                slices[row][col] = str_row[col];
                slices[col + SIZE][row] = str_row[col];
             }
        }

        Board { slices }
    }

    fn check(&self, draws: &HashSet<u8>) -> Option<u32> {
        let mut sum: u32 = 0;
        let mut winner: Option<usize> = None;

        for slice in 0..2*SIZE {
            let sum_snapshot = sum;

            for index in 0..SIZE {
                let v = self.slices[slice][index];

                if !draws.contains(&v) {
                    sum += v as u32;
                }
            }

            if winner.is_none() && sum_snapshot == sum {
                winner = Some(slice);
            }
        }

        if winner.is_some() {
            Some(sum / 2)
        } else {
            None
        }
    }
}

struct DayInput {
    lots_pool: Vec<u8>,
    boards: Vec<Board>,
    current_draws: HashSet<u8>,
    last_draw: u8,
}

impl DayInput {
    fn from_string(input: &str) -> Self {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let mut lots_pool: Vec<u8> = first_line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        let mut boards: Vec<Board> = Vec::new();

        while let Some(row) = lines.next() {
            if row.is_empty() {
                let board_data: Vec<&str> = (&mut lines).take(SIZE).collect();
                boards.push(Board::from_string(&board_data));
            }
        }

        let mut current_draws: HashSet<u8> = HashSet::from_iter(lots_pool.drain(0..3));
        let mut last_draw = lots_pool.remove(0);
        current_draws.insert(last_draw);
 
        DayInput { lots_pool, boards, current_draws, last_draw }
    }

    fn next_draw(&mut self) -> u8 {
        self.last_draw = self.lots_pool.remove(0);
        self.current_draws.insert(self.last_draw);
        self.last_draw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
    r###"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "###;

    #[test]
    pub fn test_aoc21_4_data_load() {
        let test_data = DayInput::from_string(TEST_INPUT);

        assert_eq!(test_data.lots_pool.len(), 27);
        assert_eq!(test_data.boards.len(), 3);
        assert_eq!(test_data.boards[2].slices[0][2], 17u8);
        assert_eq!(test_data.boards[2].slices[2*SIZE - 1][2], 20u8);
    }

    #[test]
    pub fn test_aoc21_4_p1() {
        let mut test_data = DayInput::from_string(TEST_INPUT);

        let mut current_lots: HashSet<u8> = HashSet::from_iter(test_data.lots_pool.drain(0..3));
        let mut result: u32 = 0;
        let mut last_draw = test_data.lots_pool.remove(0);
        current_lots.insert(last_draw);

        loop {
            let winner_sum = test_data.boards.iter().map(|b| b.check(&current_lots)).find(|e| e.is_some());
            
            if let Some(Some(sum)) = winner_sum {
                result = sum * last_draw as u32;
                break;
            } else if test_data.lots_pool.is_empty() {
                log::error!("No winner board were found"); 
                break;
            } else {
                last_draw = test_data.lots_pool.remove(0);
                current_lots.insert(last_draw);
                continue;
            }
        }

        assert_eq!(last_draw, 24);
        assert_eq!(result, 4512);
    }

    #[test]
    pub fn test_aoc21_4_p2() {
        let mut data = DayInput::from_string(TEST_INPUT);

        let mut result: u32 = 0;
        let mut winner_board_idx: usize = 0;

        loop {
            let not_winner_boards_indices: Vec<usize> = data.boards.iter()
                .map(|b| b.check(&data.current_draws))
                .enumerate().filter(|(_, e)| e.is_none())
                .map(|(i, _)| i)
                .collect();

            if not_winner_boards_indices.len() == 1 {
                winner_board_idx = not_winner_boards_indices[0];
                break;
            } else {
                data.next_draw();
            }
        }

        loop {
            if let Some(sum) = data.boards[winner_board_idx].check(&data.current_draws) {
                result = sum * data.last_draw as u32;
                break;
            } else if data.lots_pool.is_empty() {
                log::error!("Finished without reaching expected condition");
                break;
            } else {
                data.next_draw();
            }
        }

        assert_eq!(data.last_draw, 13);
        assert_eq!(result, 1924);
    }
}

impl Solution for TheDay {
    type Output = u32;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let mut data = DayInput::from_string(input);

        let mut current_lots: HashSet<u8> = HashSet::from_iter(data.lots_pool.drain(0..3));
        let mut result: u32 = 0;
        let mut last_draw = data.lots_pool.remove(0);
        current_lots.insert(last_draw);

        loop {
            let winner_sum = data.boards.iter().map(|b| b.check(&current_lots)).find(|e| e.is_some());
            
            if let Some(Some(sum)) = winner_sum {
                result = sum * last_draw as u32;
                break;
            } else if data.lots_pool.is_empty() {
                log::error!("No winner board were found"); 
                break;
            } else {
                last_draw = data.lots_pool.remove(0);
                current_lots.insert(last_draw);
                continue;
            }
        }

        Ok(result)
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let mut data = DayInput::from_string(input);

        let mut result: u32 = 0;
        let mut winner_board_idx: usize = 0;

        loop {
            let not_winner_boards_indices: Vec<usize> = data.boards.iter()
                .map(|b| b.check(&data.current_draws))
                .enumerate().filter(|(_, e)| e.is_none())
                .map(|(i, _)| i)
                .collect();

            if not_winner_boards_indices.len() == 1 {
                winner_board_idx = not_winner_boards_indices[0];
                break;
            } else {
                data.next_draw();
            }
        }

        loop {
            if let Some(sum) = data.boards[winner_board_idx].check(&data.current_draws) {
                result = sum * data.last_draw as u32;
                break;
            } else if data.lots_pool.is_empty() {
                log::error!("Finished without reaching expected condition");
                break;
            } else {
                data.next_draw();
            }
        }

        Ok(result)
    }
}
