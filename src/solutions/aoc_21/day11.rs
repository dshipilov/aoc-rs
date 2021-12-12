use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

use std::collections::HashSet;

pub type TheDay = Day<YEAR, 11>;

struct EnergyMap {
    energy: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl EnergyMap {
    fn from_string(data: &str) -> Self {
        let columns = data.trim().lines().next().unwrap().len();
        let energy: Vec<u8> = 
            data.lines()
                .filter(|s| !s.is_empty())
                .flat_map(|l| l.trim().chars().map(|l| l.to_digit(10).unwrap() as u8))
                .collect();
        let rows = energy.len() / columns; 

        Self { energy, columns, rows }
    }

    fn from_index(&self, index: usize) -> (usize, usize) { // (row, column)
        (index / self.columns, index % self.columns)
    }

    fn to_index(&self, row: usize, col: usize) -> usize {
        use std::cmp::{min, max};

        let r = min(max(0, row), self.rows - 1);
        let c = min(max(0, col), self.columns - 1);

        r * self.columns + c
    }

    fn neighbors(&self, index: usize) -> Vec<usize> {
        let last_col: usize = self.columns - 1;
        let last_row: usize = self.rows - 1;

        match self.from_index(index) {
            // Corners:
            // - NW
            (0, 0) => vec![self.to_index(0, 1), self.to_index(1, 1), self.to_index(1, 0)],
            // - SW
            (r, 0) if r == last_row => vec![self.to_index(r - 1, 0), self.to_index(r - 1, 1), self.to_index(r, 1)],
            // - NE
            (0, c) if c == last_col => vec![self.to_index(0, c - 1), self.to_index(1, c), self.to_index(1, c - 1)],
            // - SE
            (r, c) if r == last_row && c == last_col => vec![self.to_index(r, c - 1), self.to_index(r - 1, c - 1), self.to_index(r - 1, c)],

            // Edges:
            // - W
            (r, 0) => vec![self.to_index(r - 1, 0), self.to_index(r - 1, 1), self.to_index(r, 1), self.to_index(r + 1, 1), self.to_index(r + 1, 0)],
            // - N
            (0, c) => vec![self.to_index(0, c - 1), self.to_index(0, c + 1), self.to_index(1, c + 1), self.to_index(1, c), self.to_index(1, c - 1)],
            // - E 
            (r, c) if c == last_col => vec![self.to_index(r, c - 1), self.to_index(r - 1, c - 1), self.to_index(r - 1, c), self.to_index(r + 1, c), self.to_index(r + 1, c - 1)],
            // - S
            (r, c) if r == last_row => vec![self.to_index(r, c - 1), self.to_index(r - 1, c - 1), self.to_index(r - 1, c), self.to_index(r - 1, c + 1), self.to_index(r, c + 1)],
            // Inner
            (r, c) => vec![self.to_index(r, c - 1), self.to_index(r - 1, c - 1), self.to_index(r - 1, c), self.to_index(r - 1, c + 1),
                           self.to_index(r, c + 1), self.to_index(r + 1, c + 1), self.to_index(r + 1, c), self.to_index(r + 1, c - 1)]    
        }
    }

    fn step(&mut self) -> usize {
        let mut flashed: HashSet<usize> = HashSet::new();
        let mut to_scan: Vec<usize> = (0..self.energy.len()).collect();

        while to_scan.len() > 0 {
            let mut scan_neighbors: Vec<usize> = Vec::new();

            for i in to_scan.drain(..) {
                if !flashed.contains(&i) {
                    self.energy[i] += 1;

                    if self.energy[i] > 9 {
                        self.energy[i] = 0;

                        flashed.insert(i);
                        scan_neighbors.push(i);
                    }
                }
            }            
            to_scan = scan_neighbors.iter()
                .flat_map(|idx| self.neighbors(*idx))
                .filter(|idx| !flashed.contains(idx))
                .collect();
        }

        flashed.len()
    }
}

impl std::fmt::Display for EnergyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, e) in self.energy.iter().enumerate() {
            write!(f, "{}", e)?;
            let idx = i + 1;
            if idx % self.columns == 0 {

                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let mut m = EnergyMap::from_string(input);
        let mut flashed = 0;

        for _ in 0..100 {
            flashed += m.step();
        }

        Ok(flashed)
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let mut m = EnergyMap::from_string(input);
        let mut step = 0;

        while !m.energy.iter().all(|e| *e == 0u8) {
            m.step();

            step += 1;
        }

        Ok(step)
    }
}
