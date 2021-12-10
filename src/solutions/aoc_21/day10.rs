use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 10>;

#[cfg(test)]
mod tests {
    use super::*;
    use Validation::*;

    #[test]
    pub fn test_day21_10() {
        assert_eq!(validate("{([(<{}[<>[]}>{[]{[(<()>"), Invalid('}'));
        assert_eq!(validate("[[<[([]))<([[{}[[()]]]"), Invalid(')'));
        assert_eq!(validate("[({(<(())[]>[[{[]{<()<>>"), Valid("[({([[{{".chars().collect::<Vec<char>>()));
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Validation {
    Valid(Vec<char>),
    Invalid(char),
}

fn pair(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unexpected char: {}", c), 
    }
}

fn validate(s: &str) -> Validation {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if "({[<".contains(c) {
            stack.push(c);
        } else {
            if pair(stack.pop().unwrap()) != c {
                return Validation::Invalid(c);
            }
        }
    }
    Validation::Valid(stack) 
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        use Validation::Invalid;

        Ok(
            input.lines()
                .map(|s| match validate(s) {
                    Invalid(')') => 3,
                    Invalid(']') => 57,
                    Invalid('}') => 1197,
                    Invalid('>') => 25137,
                    _ => 0
            })
            .sum()
        )
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        let mut scores: Vec<usize> = input.lines()
            .map(|l| {
                match validate(l) {
                    Validation::Valid(stack) => {
                        stack.iter()
                            .rev()
                            .map(|c| match pair(*c) {
                                ')' => 1,
                                ']' => 2,
                                '}' => 3,
                                '>' => 4,
                                 _ => 0 
                             })
                            .fold(0, |acc, v| 5 * acc + v)
                        },
                    _ => 0
                }
            })
            .filter(|v| *v > 0)
            .collect();

        scores.sort_unstable();

        Ok(scores[scores.len() / 2])
    }
}
