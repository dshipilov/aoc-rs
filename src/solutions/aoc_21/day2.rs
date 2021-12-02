use anyhow::Result;
use crate::common::{ Solution, Day };
use regex::Regex;
use lazy_static::lazy_static;
use super::YEAR;

pub type TheDay = Day<YEAR, 2>;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Command {
    fn from_string(s: &str) -> Command {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"^(forward|down|up)\s(\d+)$").unwrap();
        }

        let caps = PATTERN.captures(s).unwrap(); 
        let arg = caps.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    
        match caps.get(1).map_or("", |m| m.as_str()) {
            "forward" => Command::Forward(arg),
            "up" => Command::Up(arg),
            "down" => Command::Down(arg),
            _ => panic!("Unexpected input"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().into_iter()
    .map(|s| Command::from_string(s))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_command_parsing() {
        assert_eq!(Command::from_string("forward 3"), Command::Forward(3));
        assert_eq!(Command::from_string("up 10"), Command::Up(10));
        assert_eq!(Command::from_string("down 5"), Command::Down(5));
    }
}

impl Solution for TheDay {
    type Output = i32;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        let mut horiz = 0;
        let mut depth = 0;
        
        for cmd in parse_input(input).iter() {
            match cmd {
                Command::Forward(n) => horiz += n,
                Command::Up(n) => depth -= n,
                Command::Down(n) => depth += n,
            }
        }
        
        Ok(horiz * depth)
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let mut horiz = 0;
        let mut depth = 0;
        let mut aim = 0;
        
        for cmd in parse_input(input).iter() {
            match cmd {
                Command::Forward(n) => {
                    horiz += n;
                    depth += aim * n;
                },
                Command::Up(n) => aim -= n,
                Command::Down(n) => aim += n,
            }
        }
        
        Ok(horiz * depth)
    }
}
