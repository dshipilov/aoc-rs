extern crate reqwest;
extern crate chrono;

use anyhow::{ Result, Context, anyhow };
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use reqwest::StatusCode;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use log::{info, error, debug};
use chrono::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct AocDate {
    year: u32,
    day: u32,
}

pub struct Day<const Y: u32, const D: u32> {
    date: AocDate,
    input: Option<String>,
}

impl fmt::Display for AocDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.year, self.day)
    }
}

impl AocDate {
    pub fn today() -> AocDate {
        let dt = Local::now();

        AocDate { year: dt.year() as u32, day: dt.day() as u32 }
    }

    fn url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }

    fn file_path(&self) -> PathBuf {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("input");
        path_buf.push(self.year.to_string());
        path_buf.push(format!("day{}.txt", self.day));

        debug!("File for {}: {}", self.to_string(), path_buf.to_string_lossy());

        path_buf
    }

    fn load_data(&self) -> Result<String> {
        let input_path = self.file_path();
        std::fs::create_dir_all(input_path.parent().unwrap())?;
        let mut input = String::new();

        if input_path.exists() {
            info!("Reading day {} input from file {}", self, input_path.to_string_lossy());
            let f = File::open(input_path).context("Cannot open input data file")?;
            let mut reader = BufReader::new(f);

            reader.read_to_string(&mut input).context("Unable to read input data file contents")?;
        } else {
            if let Some(token) = option_env!("AOC_TOKEN") {
                let client = Client::new();
                let token_header = format!("session={}", token);
                let url = self.url();

                let response = client
                    .get(&url)
                    .header(COOKIE, token_header)
                    .send()?;

                match response.status() {
                    StatusCode::OK => {
                        let body_text = response
                        .text()
                        .context("Unable to download input data file")?;

                        let mut file = File::create(&input_path)?;
                
                        file.write_all(body_text.as_bytes())
                        .context("Unable to store downloaded input data")?;

                        input.replace_range(.., &body_text);
                     },
                    code =>
                        return Err(anyhow!("Got unexpected response with code {}", code))
                }

           } else {
                error!("Please, set your AoC token with AOC_TOKEN environment variable");
                return Err(anyhow!("AoC token not set"));
            }
        }

        Ok(input)
    }
}

pub trait Solution {
    fn part1(&mut self) -> Result<String>;
    fn part2(&mut self) -> Result<String>;
}

pub trait Solvable {
    fn solve(&mut self) -> (Option<String>, Option<String>);
}

impl<const Y: u32, const D: u32> Solvable for Day<Y, D> where Day<Y,D>: Solution {
    fn solve(&mut self) -> (Option<String>, Option<String>) {
        if self.input.is_none() {
            match self.date.load_data() {
                Ok(input) =>
                    self.input = Some(input),
                Err(error) =>
                    panic!("Unable to load input data: {}", error)
            }
        }

        match self.part1() {
            Err(error) => {
                error!("Error solving part 1: {}", error);
                (None, None)
            },
            Ok(result1) => {
                match self.part2() {
                    Err(error) => {
                        error!("Error solving part 2: {}", error);
                        (Some(result1), None)
                    },
                    Ok(result2) =>
                        (Some(result1), Some(result2))
                }
            }
        }
    }
}

impl<const Y: u32, const D: u32> Default for Day<Y, D> {
    fn default() -> Self {
        Day { date: AocDate { year: Y, day: D }, input: None }
    }
}

