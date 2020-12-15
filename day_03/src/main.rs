/*!
# Advent of Code 2020 - Day 03
[Link to task.](https://adventofcode.com/2020/day/3)

How many trees do you encounter on your journey? Try multiple routes
and multiply their tree-counts together to get the answer.

Starting from top-left corner (x=0, y=0) and using the following map
(which repeats infinitely sideways), where # represents a tree:

```text ignore
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
```

You travel 3 steps right and 1 step left. If the position where arrive
is a tree, increase the count of trees. Continue until you have arrived
on the lowest line (y=10) on the map.

Repeat for different travel patterns. Multiply all resulting counts of
trees together.


## Usage example

```text ignore
PS> cargo run --bin day_03
   Compiling day_03 v0.1.0 (...\advent_of_code_2020\day_03)
    Finished dev [unoptimized + debuginfo] target(s) in 1.68s
     Running `target\debug\day_03.exe`
Advent of Code 2020 - Day 03
Info: Using hard-coded test data. ".aoc-session" not found.
Answer: 336 trees encountered while travelling.
```

## Notes / TODO

After implementing this solution I realized that it would be possible
to just get .lines().nth(n) and .chars().nth(n) of the input and then
compare the character at that position to see whether to add to tree
count or not. This would've been much more elegant solution, atleast
on memory usage persepective.

This is implementation is complex enough that tests should be added.
!*/

use anyhow::{bail, Result};
use reqwest;
use std::fs::read_to_string;
use std::path::Path;

static AOC_URL: &'static str = "https://adventofcode.com/2020/day/3/input";
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
        "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#",
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

struct Coords {
    x: isize,
    y: isize,
}
struct TobogganMap {
    map: Vec<Vec<usize>>,
    pos: Coords,
    max_x: isize,
    max_y: isize,
}

impl TobogganMap {
    /// Create map instance from string representation of the map.
    pub fn from_string_map(mapstr: &str) -> TobogganMap {
        let mut map = Vec::new();
        let mut max_row: isize = 0;
        let mut max_col: isize = 0;

        // Loop through the string map row by row. Add 1 to that position
        // on the map if tree is encountered.
        for (rownum, row) in mapstr.lines().enumerate() {
            map.push(Vec::new());
            for (colnum, col) in row.trim().chars().enumerate() {
                map[rownum].push(match col {
                    '#' => 1,
                    _ => 0,
                });
                if colnum as isize > max_col {
                    max_col = colnum as isize
                };
            }
            if rownum as isize > max_row {
                max_row = rownum as isize
            };
        }

        TobogganMap {
            map: map,
            pos: Coords { x: 0, y: 0 },
            max_x: max_col,
            max_y: max_row,
        }
    }

    /// Move on the map and return the value on the new position.
    /// Map will repeat infinitely on x-axis, but not on y.
    pub fn move_by(&mut self, x: isize, y: isize) -> Result<usize> {
        // Check for x overflow, which is allowed for 'infinite scrolling'.
        if self.pos.x + x > self.max_x {
            self.pos.x += x - self.max_x - 1;
        }
        // Check for x underflow, which is allowed for 'infinite scrolling'.
        else if self.pos.x + x < 0 {
            self.pos.x += x + self.max_x + 1;
        }
        // Just move
        else {
            self.pos.x += x;
        }

        // Check for y overflow, which is not allowed.
        if self.pos.y + y > self.max_y {
            bail!("Illegal move")
        }
        // Check for y underflow, which is not allowed.
        else if self.pos.y + y < 0 {
            bail!("Illegal move")
        }
        // Just move
        else {
            self.pos.y += y;
        }

        // Return value at this pos.
        return Ok(self.map[self.pos.y as usize][self.pos.x as usize]);
    }

    pub fn reset_position(&mut self) {
        self.pos.x = 0;
        self.pos.y = 0;
    }
}

fn main() {
    println!("Advent of Code 2020 - Day 03");
    let map_data = get_input();
    let mut map = TobogganMap::from_string_map(&map_data);
    let mut all_tree_counts: Vec<usize> = Vec::new();
    let mut encountered_trees: usize = 0;

    // Move until end of map for all slopes
    loop {
        match map.move_by(1, 1) {
            Ok(val) => encountered_trees += val,
            Err(_) => break,
        }
    }
    all_tree_counts.push(encountered_trees);
    encountered_trees = 0;
    map.reset_position();

    loop {
        match map.move_by(3, 1) {
            Ok(val) => encountered_trees += val,
            Err(_) => break,
        }
    }
    all_tree_counts.push(encountered_trees);
    encountered_trees = 0;
    map.reset_position();

    loop {
        match map.move_by(5, 1) {
            Ok(val) => encountered_trees += val,
            Err(_) => break,
        }
    }
    all_tree_counts.push(encountered_trees);
    encountered_trees = 0;
    map.reset_position();

    loop {
        match map.move_by(7, 1) {
            Ok(val) => encountered_trees += val,
            Err(_) => break,
        }
    }
    all_tree_counts.push(encountered_trees);
    encountered_trees = 0;
    map.reset_position();

    loop {
        match map.move_by(1, 2) {
            Ok(val) => encountered_trees += val,
            Err(_) => break,
        }
    }
    all_tree_counts.push(encountered_trees);
    encountered_trees = 0;
    map.reset_position();

    // Calculate answer by multiplying all counts together
    for count in all_tree_counts.iter() {
        if encountered_trees == 0 {
            encountered_trees = count.clone();
        } else {
            encountered_trees = encountered_trees * count;
        }
    }

    println!(
        "Answer: {} trees encountered while travelling.",
        encountered_trees
    );
}

#[cfg(test)]
mod day_03 {
    use super::*;

    #[test]
    fn run() {
        main();
    }
}
