use anyhow::Result;
use crate::common::{ Solution, Day };
use super::YEAR;

pub type TheDay = Day<YEAR, 8>;

fn digit_to_byte(digit: &str) -> u8 {
    digit
        .chars()
        .map(|c| 
            match c {
                'a' => 0b0000001,
                'b' => 0b0000010,
                'c' => 0b0000100,
                'd' => 0b0001000,
                'e' => 0b0010000,
                'f' => 0b0100000,
                'g' => 0b1000000,
                _ => panic!("Unexpected char: {}", c)
            }
        ).reduce(|a, b| a | b).unwrap()
}

struct Decoder {
    four: u8,
    seven: u8,
}

impl Decoder {
    fn from_string(data: &str) -> Self {
        let digits: Vec<(usize, u8)> = data.trim()
            .split(" ")
            .map(|s| (s.len(), digit_to_byte(s)))
            .collect();

        let four = digits.iter().find(|(l,_)| *l == 4).map(|(_,v)| *v).unwrap();
        let seven = digits.iter().find(|(l,_)| *l == 3).map(|(_,v)| *v).unwrap();

        Self { four, seven }
    }

    fn decode(&self, digit: &str) -> usize {
        fn intersect_len(a: u8, b: u8) -> usize {
            (a & b).count_ones() as usize
        }

        match (digit.len(), digit_to_byte(digit)) {
            (2, _) => 1,
            (3, _) => 7,
            (4, _) => 4,
            (5, d) if intersect_len(d, self.seven) == 3 => 3,
            (5, d) if intersect_len(d, self.four ) == 3 => 5,
            (5, d) if intersect_len(d, self.four ) == 2 => 2,
            (6, d) if intersect_len(d, self.four ) == 4 => 9,
            (6, d) if intersect_len(d, self.seven) == 3 => 0,
            (6, d) if intersect_len(d, self.four ) == 3 => 6, 
            (7, _) => 8,
            (l, d) => panic!("Unexpected decoding condition: len = {}, digit = {:07b}", l, d), 
        }
    }

    fn decode_number(&self, number: &str) -> usize {
        let digits: Vec<&str> = number.trim().split(" ").collect();

        digits.iter()
            .map(|s| self.decode(s))
            .enumerate()
            .map(|(i, v)| v * 10_usize.pow((digits.len() - i - 1).try_into().unwrap()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_aoc21_8_decoder() {
        let decoder = Decoder::from_string("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");

        assert_eq!(decoder.decode("ba"), 1);
        assert_eq!(decoder.decode("acedbfg"), 8);
        assert_eq!(decoder.decode("fabe"), 4);
        assert_eq!(decoder.decode("dab"), 7);
        assert_eq!(decoder.decode("gcdfa"), 2);
        assert_eq!(decoder.decode("fbcad"), 3);
        assert_eq!(decoder.decode("cdfbe"), 5);
        assert_eq!(decoder.decode("cdfgeb"), 6);
        assert_eq!(decoder.decode("cefabd"), 9);
        assert_eq!(decoder.decode("cagedb"), 0);

        assert_eq!(decoder.decode_number("cdfeb fcadb cdfeb cdbaf"), 5353);
    }
}

impl Solution for TheDay {
    type Output = usize;

    fn part1(&self, input: &str) -> Result<Self::Output> {
        Ok( 
            input.lines()
                .map(|s| s.split(" ")
                          .skip_while(|s| *s != "|")
                          .skip(1)
                          .map(|s| s.len())
                          .filter(|l| *l == 2 || *l == 3 || *l == 4 || *l == 7)
                          .count())
                .sum()
        )
    }

    fn part2(&self, input: &str) -> Result<Self::Output> {
        Ok(
            input.lines()
            .map(|s| {
                let (digits, query) = s.split_at(s.find("|").unwrap());
                Decoder::from_string(digits).decode_number(query.replace("|", "").trim())
            })
            .sum()
        )
    }
}
