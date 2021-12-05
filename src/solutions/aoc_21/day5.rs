use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 5>;

use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Debug, PartialEq, Eq)]
struct Segment {
    begin: Point,
    end: Point,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point{ x, y }
    }

    fn is_on_same_vertical(&self, p: &Point) -> bool {
        self.x == p.x
    }

    fn is_on_same_horizontal(&self, p: &Point) -> bool {
        self.y == p.y
    }
}

impl Segment {
    fn from_string(s: &str) -> Self {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"^(\d+),(\d+)\s+->\s+(\d+),(\d+)$").unwrap();
        }

        let caps = PATTERN.captures(s).unwrap();

        let x1 = caps.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
        let y1 = caps.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
        let x2 = caps.get(3).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
        let y2 = caps.get(4).map_or(0, |m| m.as_str().parse::<u32>().unwrap());

        Segment { begin: Point { x: x1, y: y1 }, end: Point { x: x2, y: y2 } }
    }

    fn is_horizontal(&self) -> bool {
        self.begin.is_on_same_horizontal(&self.end)
    }

    fn is_vertical(&self) -> bool {
        self.begin.is_on_same_vertical(&self.end)
    }

    fn is_diagonal(&self) -> bool {
        i32::abs(self.end.x as i32 - self.begin.x as i32) ==
        i32::abs(self.end.y as i32 - self.begin.y as i32)
    }

    fn trace<'a>(&'a self) -> SegmentTrace<'a> {
        SegmentTrace { segment: self, pos: Some(self.begin) } 
    }
}

struct SegmentTrace<'a> {
    segment: &'a Segment,
    pos: Option<Point>,
}

impl Iterator for SegmentTrace<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos {
            Some(p) if p == self.segment.end => {
                self.pos = None;
                Some(p)
            },
            Some(p) if self.segment.is_horizontal() && self.segment.begin.x < self.segment.end.x => {
                self.pos = Some(Point::new(p.x + 1, p.y));
                Some(p) 
            },
            Some(p) if self.segment.is_horizontal() && self.segment.begin.x > self.segment.end.x => {
                self.pos = Some(Point::new(p.x - 1, p.y));
                Some(p) 
            },
            Some(p) if self.segment.is_vertical() && self.segment.begin.y < self.segment.end.y => {
                self.pos = Some(Point::new(p.x, p.y + 1));
                Some(p)
            },
            Some(p) if self.segment.is_vertical() && self.segment.begin.y > self.segment.end.y => {
                self.pos = Some(Point::new(p.x, p.y - 1));
                Some(p)
            },
            Some(p) if self.segment.is_diagonal() => {
                let x = if self.segment.begin.x < self.segment.end.x {
                    p.x + 1
                } else {
                    p.x - 1
                };

                let y = if self.segment.begin.y < self.segment.end.y {
                    p.y + 1
                } else {
                    p.y - 1
                };
                
                self.pos = Some(Point::new(x, y));
                Some(p)
            },
             _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_aoc21_5_segment_iterator() {
        let horizontal = Segment::from_string("12,5 -> 14,5");
        let vertical = Segment::from_string("3,4 -> 3,2");
        let diag1 = Segment::from_string("3,3 -> 0,0");
        let diag2 = Segment::from_string("3,3 -> 5,1");
        let mut h_trace = horizontal.trace();

        assert_eq!(h_trace.next(), Some(Point::new(12, 5)));
        assert_eq!(h_trace.next(), Some(Point::new(13, 5)));
        assert_eq!(h_trace.next(), Some(Point::new(14, 5)));
        assert_eq!(h_trace.next(), None);

        let mut v_trace = vertical.trace();
        assert_eq!(v_trace.next(), Some(Point::new(3, 4)));
        assert_eq!(v_trace.next(), Some(Point::new(3, 3)));
        assert_eq!(v_trace.next(), Some(Point::new(3, 2)));
        assert_eq!(v_trace.next(), None);

        let mut d1_trace = diag1.trace();
        assert_eq!(d1_trace.next(), Some(Point::new(3, 3)));
        assert_eq!(d1_trace.next(), Some(Point::new(2, 2)));
        assert_eq!(d1_trace.next(), Some(Point::new(1, 1)));
        assert_eq!(d1_trace.next(), Some(Point::new(0, 0)));
        assert_eq!(d1_trace.next(), None);

        let mut d2_trace = diag2.trace();
        assert_eq!(d2_trace.next(), Some(Point::new(3, 3)));
        assert_eq!(d2_trace.next(), Some(Point::new(4, 2)));
        assert_eq!(d2_trace.next(), Some(Point::new(5, 1)));
        assert_eq!(d2_trace.next(), None);
    }
}

fn compute_segment_intersects(segments: &[Segment]) -> usize {
    let mut reg: HashMap<Point, u32> = HashMap::new();

    for seg in segments {
        for p in seg.trace() {
            let current_count: u32 = *reg.get(&p).unwrap_or(&0);
            reg.insert(p, current_count + 1);
        } 
    }

    let result: usize = reg.into_values()
        .filter(|v| *v > 1)
        .count();

    result
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let segments: Vec<Segment> = input.lines()
            .map(|s| Segment::from_string(s))
            .filter(|seg| seg.is_vertical() || seg.is_horizontal())
            .collect();

        Ok(compute_segment_intersects(&segments))
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let segments: Vec<Segment> = input.lines()
            .map(|s| Segment::from_string(s))
            .collect();

        Ok(compute_segment_intersects(&segments))
    }
}
