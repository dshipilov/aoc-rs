use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

use std::collections::{ HashMap, HashSet };
use std::cell::RefCell;
use std::rc::Rc;

pub type TheDay = Day<YEAR, 9>;

struct HeightMap {
    levels: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl HeightMap {
    fn from_string(data: &str) -> Self {
        let columns = data.trim().lines().next().unwrap().len();
        let levels: Vec<u8> = 
            data.lines()
                .flat_map(|l| l.trim().chars().map(|l| l.to_digit(10).unwrap() as u8))
                .collect();
        let rows = levels.len() / columns; 

        Self { levels, columns, rows }
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
            (0, 0) => vec![self.to_index(0, 1), self.to_index(1, 0)],
            // - SW
            (r, 0) if r == last_row => vec![self.to_index(r - 1, 0), self.to_index(r, 1)],
            // - NE
            (0, c) if c == last_col => vec![self.to_index(0, c - 1), self.to_index(1, c)],
            // - SE
            (r, c) if r == last_row && c == last_col => vec![self.to_index(r, c - 1), self.to_index(r - 1, c)],

            // Edges:
            // - W
            (r, 0) => vec![self.to_index(r - 1, 0), self.to_index(r, 1), self.to_index(r + 1, 0)],
            // - N
            (0, c) => vec![self.to_index(0, c - 1), self.to_index(0, c + 1), self.to_index(1, c)],
            // - E 
            (r, c) if c == last_col => vec![self.to_index(r, c - 1), self.to_index(r - 1, c), self.to_index(r + 1, c)],
            // - S
            (r, c) if r == last_row => vec![self.to_index(r, c - 1), self.to_index(r - 1, c), self.to_index(r, c + 1)],
            // Inner
            (r, c) => vec![self.to_index(r, c - 1), self.to_index(r - 1, c), self.to_index(r, c + 1), self.to_index(r + 1, c)],    
        }
    }

    fn neighbor_levels(&self, index: usize) -> Vec<u8> {
        self.neighbors(index).iter().map(|i| self.levels[*i]).collect()
    }

    fn is_lowest(&self, i: usize) -> bool {
        self.neighbor_levels(i).iter().all(|nl| *nl > self.levels[i]) 
    }

    fn trace(&self, index: usize) -> (usize, Vec<usize>) {
        let mut trace: Vec<usize> = Vec::new();
        let mut i = index;
        
        loop {
            let level = self.levels[i];
            let mut neighbors: Vec<(usize, u8)> =
                self.neighbors(i)
                    .iter().map(|i| (*i, self.levels[*i]))
                    .collect();
        
            trace.push(i);

            if neighbors.iter().map(|(_,l)| *l).all(|nl| nl > level) {
                return (i, trace)
            }
    
            neighbors.sort_unstable_by_key(|(_, l)| *l);
            i = neighbors[0].0;
         }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = 
        r###"2199943210
             3987894921
             9856789892
             8767896789
             9899965678"###;

    #[test]
    pub fn test_aoc21_9_p1() {
        let hm = HeightMap::from_string(TEST_DATA);
        assert_eq!(hm.columns, 10);
        assert_eq!(hm.rows, 5);
        assert_eq!(hm.levels[49], 8u8);
        assert_eq!(hm.levels[20], 9u8);

        assert_eq!(hm.neighbor_levels(0), vec![1, 3]);
        assert_eq!(hm.neighbor_levels(hm.to_index(0, 5)), vec![9, 3, 9]);
        assert_eq!(hm.neighbor_levels(hm.to_index(hm.rows - 1, 2)), vec![8, 6, 9]);
        assert_eq!(hm.neighbor_levels(hm.to_index(2, 5)), vec![7, 9, 9, 9]);
    } 
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let hm = HeightMap::from_string(input);
        let mut low_points: Vec<usize> = Vec::new();

        for (i, level) in hm.levels.iter().enumerate() {
            if hm.is_lowest(i) {
                low_points.push(*level as usize);
            }
        }

        Ok(low_points.iter().sum::<usize>() + low_points.len())
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let hm = HeightMap::from_string(input);
        let mut basins: HashMap<usize, Rc<RefCell<HashSet<usize>>>> = HashMap::new();

        for (i, level) in hm.levels.iter().enumerate() {
            if *level < 9 {
                let (point, path) = hm.trace(i);

                if !basins.contains_key(&point) {
                    basins.insert(point, Rc::new(RefCell::new(HashSet::new())));
                }

                basins[&point].borrow_mut().extend(path.iter());
            }
        }

        let mut sizes: Vec<usize> = basins.values().map(|points| points.borrow().len()).collect();
        sizes.sort_unstable();

        Ok(sizes.iter().rev().take(3).product())
    }
}
