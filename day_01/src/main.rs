/*!
# Advent of Code 2020 - Day 01
[Link to task.](https://adventofcode.com/2020/day/1)

Find two values from list where:

```text ignore
value_1 + value_2 == 2020
```

Correct answer for website is calculated by:

```text ignore
value_1 * value_2.
```

## Usage example

```text ignore
PS> cargo run --bin day_01
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
    Running `target\debug\day_01.exe`
Advent of Code 2020 - Day 01
Info: Using hard-coded test data. ".aoc-session" not found.
Correct values: 979 + 366 + 675 = 2020.
Answer: 241861950.
```
!*/

use anyhow::{bail, Result};
use reqwest;
use std::fs::read_to_string;
use std::path::Path;

static AOC_URL: &'static str = "https://adventofcode.com/2020/day/1/input";
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
        "1721
        979
        366
        299
        675
        1456",
    )
    .to_owned()
}

/// Get input data either from AOC website or fall-back to local
/// hard-coded test data.
pub fn get_input() -> Vec<u32> {
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

    // Input data is included in the binary.
    let data = input
        .lines()
        .map(|s| {
            s.trim()
                .parse::<u32>()
                .expect("Input data contained non-number value.")
        })
        .collect();
    return data;
}

/// Calculate correct answer. Uses brute force search.
pub fn day_01(input: Vec<u32>) -> (u32, u32) {
    for val1 in input.iter() {
        for val2 in input.iter() {
            if val1 + val2 == 2020 {
                return (val1.clone(), val2.clone());
            }
        }
    }
    return (0, 0);
}

fn main() {
    println!("Advent of Code 2020 - Day 01");

    // Get input data

    let (val1, val2) = day_01(get_input());
    assert_eq!(val1 + val2, 2020);
    let answer = val1 * val2;

    println!("Correct values: {} + {} = 2020.", val1, val2);
    println!("Answer: {}.", answer);
}

#[cfg(test)]
mod day_01 {
    use super::*;

    #[test]
    fn run() {
        main();
    }
}
