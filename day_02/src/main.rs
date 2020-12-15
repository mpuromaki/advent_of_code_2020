/*!
# Advent of Code 2020 - Day 02
[Link to task.](https://adventofcode.com/2020/day/2)

How many password are valid based on password policies at the time?

Input file is in rows similar to "1-3 a: abcde". Number range implies how many
letters there has to be. After semicolon is the password itself. In this example
atleast 1, but at most 3, instances of letter "a" is allowed on the password "abcde".
The example password is thus valid.

Go through input data and validate all password. Count valid passwords.

## Usage example

```text ignore
PS> cargo run --bin day_02
   Compiling day_02 v0.1.0 (...\advent_of_code_2020\day_02)
    Finished dev [unoptimized + debuginfo] target(s) in 1.81s
     Running `target\debug\day_02.exe`
Advent of Code 2020 - Day 02
Info: Using hard-coded test data. ".aoc-session" not found.
Answer: 2 valid passwords in input data.
```
!*/

use anyhow::{bail, Result};
use reqwest;
use std::fs::read_to_string;
use std::path::Path;

static AOC_URL: &'static str = "https://adventofcode.com/2020/day/2/input";
static AOC_SESSION_FILE: &'static str = ".aoc-session";

#[derive(Debug)]
pub struct PassPolicy {
    required_letter: char,
    pos_1: u32,
    pos_2: u32,
}

#[derive(Debug)]
pub struct PassInstance {
    policy: PassPolicy,
    password: String,
}

impl PassInstance {
    fn from_string(txt: &str) -> PassInstance {
        // Split string to amount, required letter and password parts.
        let parts: Vec<&str> = txt.split_whitespace().map(|s| s.into()).collect();
        let charpos: Vec<&str> = parts[0].split("-").collect();
        assert_eq!(parts.len(), 3);

        PassInstance {
            policy: PassPolicy {
                required_letter: parts[1].replace(":", "").chars().nth(0).unwrap(),
                pos_1: charpos[0].parse().expect("Failed to parse password policy"),
                pos_2: charpos[1].parse().expect("failed to parse password policy"),
            },
            password: parts[2].to_owned(),
        }
    }

    fn is_valid(&self) -> bool {
        let req1: bool = match self.password.chars().nth(self.policy.pos_1 as usize - 1) {
            Some(char) => {
                if char == self.policy.required_letter {
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        let req2: bool = match self.password.chars().nth(self.policy.pos_2 as usize - 1) {
            Some(char) => {
                if char == self.policy.required_letter {
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        // Password is valid when exactly one position is required_letter.
        return req1 ^ req2;
    }
}

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
        "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc",
    )
    .to_owned()
}

/// Get input data either from AOC website or fall-back to local
/// hard-coded test data.
pub fn get_input() -> Vec<String> {
    // Check if .aoc-session is available
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

    let data = input.lines().map(|s| s.trim().to_string()).collect();
    return data;
}

pub fn parse_input(input: Vec<String>) -> Vec<PassInstance> {
    let mut output: Vec<PassInstance> = Vec::new();
    for line in input.iter() {
        output.push(PassInstance::from_string(line));
    }
    return output;
}

pub fn count_valid_passwords(input: Vec<PassInstance>) -> u32 {
    let amount: u32 = input.iter().filter(|x| x.is_valid()).count() as u32;
    amount
}

fn main() {
    println!("Advent of Code 2020 - Day 02");
    let valid_count = count_valid_passwords(parse_input(get_input()));

    println!("Answer: {} valid passwords in input data.", valid_count);
}

#[cfg(test)]
mod day_02 {
    use super::*;

    #[test]
    fn run() {
        main();
    }
}
