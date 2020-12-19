/*!
# Advent of Code 2020 - Day 04
[Link to task.](https://adventofcode.com/2020/day/4)

Detect which passports are valid eq. have all required
fields.

Passport data is validated in batch files (your puzzle input).
Each passport is represented as a sequence of key:value pairs
separated by spaces or newlines. Passports are separated by blank
lines.

Only "cid" is allowed to be missing from otherwise valid passport.
All other fields are required.

## Usage example

```text ignore
PS> cargo run --bin day_04
   Compiling day_04 v0.1.0 (...\advent_of_code_2020\day_04)
    Finished dev [unoptimized + debuginfo] target(s) in 2.30s
     Running `target\debug\day_04.exe`
Advent of Code 2020 - Day 04
Info: Using hard-coded test data. ".aoc-session" not found.
Answer: 2 valid passports.
```

## Notes

I really wanted to use crate "uom" to handle height as proper SI-units
with proper conversion between imperial and metric. Sadly I had some
issues with the crate. F32Unit is makeshift solution, though not even
near "uom".

I also wanted to use proper RGB values for colors, but the input data
has mixed short names, hex numbers with # and hex numbers without #.
It was bit too complicated for this level of test. Thus String field.
!*/

use anyhow::{bail, Result};
use regex;
use reqwest;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

static AOC_URL: &'static str = "https://adventofcode.com/2020/day/4/input";
static AOC_SESSION_FILE: &'static str = ".aoc-session";

/// This function downloads input data from Advent of Code
/// if .aoc-session file is available and download succeeds.
fn get_input_aoc() -> Result<String> {
    let f = Path::new(&AOC_SESSION_FILE);

    if !f.is_file() {
        bail!("{:?} not found.", &AOC_SESSION_FILE);
    }

    // Load session key
    let session_key = read_to_string(f)?;

    // Load input data
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(AOC_URL)
        .header("Cookie", format!("session={}", session_key))
        .send()
        .expect("Sending request failed.");

    if response.status().is_success() {
        let resp = response.text()?;
        return Ok(resp);
    } else {
        bail!(
            "Failed to load {:?}. Response: {:?}",
            &AOC_URL,
            response.status()
        )
    }
}

/// If input data download was not available, this function
/// returns hardcoded test data which is allowed to be shared.
fn get_input_test() -> String {
    String::from(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in",
    )
    .to_owned()
}

/// Get input data either from AOC website or fall-back to local
/// hard-coded test data.
pub fn get_input() -> String {
    let input: String = match get_input_aoc() {
        Ok(data) => {
            println!("Info: Downloaded test data from: {}", AOC_URL);
            data
        }
        Err(e) => {
            println!("Info: Using hard-coded test data. {}", e);
            get_input_test()
        }
    };

    return input;
}

struct F32Unit {
    value: f32,
    unit: String,
}

impl std::str::FromStr for F32Unit {
    type Err = std::num::ParseFloatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: (&str, &str) = match input.find(|s: char| s.is_alphabetic()) {
            Some(splitpoint) => input.split_at(splitpoint),
            None => (input, ""),
        };
        let value: f32 = split.0.parse().unwrap();
        let unit: String = split.1.to_string();
        Ok(F32Unit { value, unit })
    }
}

pub struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: F32Unit,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

impl Passport {
    // Parse the input string into Passport instance.
    // Input key:value pairs are parsed to a hashmap
    // where the data is used to construct Passport.
    pub fn from_string(input: &str) -> Result<Passport> {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for item in input.split_whitespace() {
            let kv: Vec<&str> = item.split(':').collect();
            let _ = fields.insert(kv[0], kv[1]);
        }

        return Ok(Passport {
            birth_year: match fields.get("byr") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: byr"),
            },
            issue_year: match fields.get("iyr") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: iyr"),
            },
            expiration_year: match fields.get("eyr") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: eyr"),
            },
            height: match fields.get("hgt") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: hgt"),
            },
            hair_color: match fields.get("hcl") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: byr"),
            },
            eye_color: match fields.get("ecl") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: ecr"),
            },
            passport_id: match fields.get("pid") {
                Some(data) => data.parse().unwrap(),
                None => bail!("Invalid data: pid"),
            },
            country_id: match fields.get("cid") {
                Some(data) => Some(data.parse().unwrap()),
                None => None,
            },
        });
    }
}

pub fn parse_string_to_passports(input: &str) -> Vec<Passport> {
    let mut output: Vec<Passport> = Vec::new();

    // Parse input data and pass blocks of str to Passport constructor
    // if Passport returns valid passport, add it to the vec.
    let re = regex::RegexBuilder::new(r"^\s*$")
        .multi_line(true)
        .build()
        .unwrap();
    for block in re.split(input) {
        match Passport::from_string(block) {
            Ok(passport) => {
                output.push(passport);
            }
            Err(_) => {}
        }
    }

    return output;
}

fn main() {
    println!("Advent of Code 2020 - Day 04");
    let input_data = get_input();
    let passports = parse_string_to_passports(&input_data);

    println!("Answer: {} valid passports.", passports.len());
}

#[cfg(test)]
mod day_04 {
    use super::*;

    #[test]
    fn run() {
        main();
    }
}
