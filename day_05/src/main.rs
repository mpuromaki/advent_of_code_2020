/*!
# Advent of Code 2020 - Day 05
[Link to task.](https://adventofcode.com/2020/day/5)

What is the ID of your seat? Your seat wasn't at the very
front or back, though; the seats with IDs +1 and -1 from
yours will be in your list.

The seat IDs are written in binary space partition using
F, B, L & R letters. First 7 letters are either Front or
Back, where Front means lower half. These specify exactly
one of 0..127 possible rows. Last three characters are either
Left or Right, where Left means lower half. These specify
exactly one of 0..7 possible seats.

Seat ID is calculated by multiplying row by 8 and add column.

## Usage example

```text ignore
PS> cargo run --bin day_05
    Finished dev [unoptimized + debuginfo] target(s) in 1.77s
     Running `target\debug\day_05.exe`
Advent of Code 2020 - Day 05
Info: Downloaded test data from: https://adventofcode.com/2020/day/5/input
Answer: PlaneSeat { row: 65, seat: 4, id: 524 } is my seat!
```

## Notes

I wanted to try bitmasks and bit manipulations as a solution
for this binary space partitioning task.
!*/

use anyhow::{bail, Result};
use reqwest;
use std::fs::read_to_string;
use std::path::Path;

static AOC_URL: &'static str = "https://adventofcode.com/2020/day/5/input";
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
        "FBFBBFFRLR
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL",
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

#[derive(Default, Debug)]
pub struct PlaneSeat {
    row: usize,
    seat: usize,
    id: usize,
}

pub fn get_seat_id(row: usize, seat: usize) -> usize {
    return row * 8 + seat;
}

pub fn string_to_planeseat(input: &str) -> Option<PlaneSeat> {
    let mut row_mask: u8 = 0b1111111; // 127 is the highest possible row
    let mut seat_mask: u8 = 0b111; // 7 is the highest possible seat

    // We step the row_mask from left to right.
    // If we are keeping the lower value, we set mask at that index to 0.
    // Otherwise we leave the mask at 1.
    // Binary masks are complex. Here be dragons.
    for c in input.chars().enumerate() {
        match c.1 {
            'F' => row_mask = row_mask & (0b1111111 ^ 1 << (6 - c.0)), // offset 0 - 6, Set to zero
            'B' => row_mask = row_mask | (1 << (6 - c.0)),             // offset 0 - 6, Set to one
            'L' => seat_mask = seat_mask & (0b0000111 ^ 1 << (2 - (c.0 - 7))), // offset 7 - 9, Set to zero
            'R' => seat_mask = seat_mask | (1 << (2 - (c.0 - 7))), // offset 7 - 9, Set to one
            _ => break,
        }

        if c.0 == 9 {
            return Some(PlaneSeat {
                row: row_mask as usize,
                seat: seat_mask as usize,
                id: get_seat_id(row_mask as usize, seat_mask as usize),
            });
        }
        if c.0 > 9 {
            // Something is wrong
            return None;
        }
    }

    return None;
}

fn main() {
    println!("Advent of Code 2020 - Day 05");
    let input_data = get_input();

    let mut seat_list: Vec<PlaneSeat> = Vec::new();

    // Get PlaneSeats from input data
    for line in input_data.lines() {
        seat_list.push(
            string_to_planeseat(line.trim())
                .or_else(|| Some(PlaneSeat::default()))
                .unwrap(),
        )
    }

    // Get the highest Seat ID for the task answer
    seat_list.sort_unstable_by_key(|k| k.id);
    let lowest_id = seat_list.iter().nth(0).unwrap().id;
    let highest_id = seat_list.iter().nth_back(0).unwrap().id;

    // Task tells that IDs -1 and +1 from our seat are on the list.
    // Therefore we can loop once through the sorted list and find where
    // id_now - id_prev == 2. Our seat ID will be id_now -1.
    let mut prev_place: PlaneSeat = PlaneSeat::default();
    let mut my_place: PlaneSeat = PlaneSeat::default();
    for place in seat_list {
        let distance = place.id - prev_place.id;
        //println!("{:?} - {:?}", distance, place);
        if distance == 2 {
            my_place = PlaneSeat {
                row: (place.row + prev_place.row) / 2,
                seat: (place.seat + prev_place.seat) / 2,
                id: place.id - 1,
            };
            break;
        }
        // Update prev values
        prev_place.row = place.row;
        prev_place.seat = place.seat;
        prev_place.id = place.id;
    }

    println!("Answer: {:?} is my seat!", my_place);
}

#[cfg(test)]
mod day_04 {
    use super::*;

    #[test]
    fn run() {
        main();
    }
}
