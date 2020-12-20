/*!
# Advent of Code 2020 - Day 04
[Link to task.](https://adventofcode.com/2020/day/4)

Detect which passports are valid eq. have all required
fields with some limitations.

Passport data is validated in batch files (your puzzle input).
Each passport is represented as a sequence of key:value pairs
separated by spaces or newlines. Passports are separated by blank
lines.

Only "cid" is allowed to be missing from otherwise valid passport.
All other fields are required.

Fields have to validated by these rules:
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

## Usage example

```text ignore
PS> cargo run --bin day_04
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target\debug\day_04.exe`
Advent of Code 2020 - Day 04
Info: Using hard-coded test data. ".aoc-session" not found.
Answer: 2 valid passports.
```
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
    unit: Option<String>,
}

impl std::str::FromStr for F32Unit {
    type Err = std::num::ParseFloatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: (&str, &str) = match input.find(|s: char| s.is_alphabetic()) {
            Some(splitpoint) => input.split_at(splitpoint),
            None => (input, ""),
        };
        let value: f32 = split.0.parse().unwrap();
        let unit = split.1;

        if split.1 == "" {
            return Ok(F32Unit { value, unit: None });
        } else {
            return Ok(F32Unit {
                value,
                unit: Some(unit.into()),
            });
        };
    }
}

#[allow(dead_code)]
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
    // The data is validated on construction.
    pub fn from_string(input: &str) -> Result<Passport> {
        let fields = Passport::str_to_hashmap(input);

        return Ok(Passport {
            birth_year: Passport::validate_number(fields.get_key_value("byr"), 1920, 2002)?,
            issue_year: Passport::validate_number(fields.get_key_value("iyr"), 2010, 2020)?,
            expiration_year: Passport::validate_number(fields.get_key_value("eyr"), 2020, 2030)?,
            height: Passport::validate_height(
                fields.get_key_value("hgt"),
                (150.0, 193.0),
                (59.0, 76.0),
            )?,
            hair_color: Passport::validate_haircolor(fields.get_key_value("hcl"))?,
            eye_color: Passport::validate_eyecolor(fields.get_key_value("ecl"))?,
            passport_id: Passport::validate_id(fields.get_key_value("pid"))?,
            country_id: match fields.get("cid") {
                Some(data) => Some(data.parse().unwrap()),
                None => None,
            },
        });
    }

    /// Get hashmap from str input data.
    fn str_to_hashmap(input: &str) -> HashMap<&str, &str> {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for item in input.split_whitespace() {
            let kv: Vec<&str> = item.split(':').collect();
            let _ = fields.insert(kv[0], kv[1]);
        }
        return fields;
    }

    /// Extract the data from hashmap
    fn get_kv<'a>(data: Option<(&&'a str, &&'a str)>) -> Result<(&'a str, &'a str)> {
        // Get value, check it's safe.
        let (k, v) = match data {
            Some(data) => (*data.0, *data.1),
            None => bail!("Missing field."),
        };

        return Ok((k, v));
    }

    // Validate data to between low and high. If not valid, return Err early.
    fn validate_number(data: Option<(&&str, &&str)>, low: usize, high: usize) -> Result<usize> {
        let (k, v) = Passport::get_kv(data)?;

        // Parse the value to correct type
        let v = match v.parse::<usize>() {
            Ok(v) => v,
            Err(_) => bail!("Malformed field {}.", k),
        };

        // Validate the value
        if v < low {
            bail!("Invalid: {} < {}", k, low);
        };
        if v > high {
            bail!("Invalid: {} < {}", k, high);
        };
        return Ok(v);
    }

    // Validate data to between cm_low and cm_high if unit is cm.
    // Validate data to between in_low and in_high if unit is in.
    // If not valid, return Err early.
    fn validate_height(
        data: Option<(&&str, &&str)>,
        (cm_low, cm_high): (f32, f32),
        (in_low, in_high): (f32, f32),
    ) -> Result<F32Unit> {
        let (k, v) = Passport::get_kv(data)?;

        // Parse the value to correct type
        let v = match v.parse::<F32Unit>() {
            Ok(v) => v,
            Err(_) => bail!("Malformed field {}.", k),
        };

        match &v.unit {
            Some(unit) => {
                if unit == "cm" {
                    if v.value < cm_low {
                        bail!("Invalid: {} < {} cm", k, cm_low);
                    };
                    if v.value > cm_high {
                        bail!("Invalid: {} < {} cm", k, cm_high);
                    };
                } else if unit == "in" {
                    if v.value < in_low {
                        bail!("Invalid: {} < {} inch", k, in_low);
                    };
                    if v.value > in_high {
                        bail!("Invalid: {} < {} inch", k, in_high);
                    };
                } else {
                    bail!("Invalid: {} - unknown unit", k);
                }
            }
            None => bail!("Invalid: {} - no unit", k),
        }

        return Ok(v);
    }

    // Validate data to # followed by exactly six characters 0-9 or a-f.
    // If not valid, return Err early.
    fn validate_haircolor(data: Option<(&&str, &&str)>) -> Result<String> {
        let (k, v) = Passport::get_kv(data)?;

        let re = regex::Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
        if re.is_match(v) {
            return Ok(v.to_owned());
        } else {
            bail!("Invalid: {}", k);
        }
    }

    // Validate data to exactly one of: amb blu brn gry grn hzl oth.
    // If not valid, return Err early.
    fn validate_eyecolor(data: Option<(&&str, &&str)>) -> Result<String> {
        let (k, v) = Passport::get_kv(data)?;

        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v) {
            return Ok(v.to_owned());
        } else {
            bail!("Invalid: {}", k);
        }
    }

    // Validate data to exactly one of: amb blu brn gry grn hzl oth.
    // If not valid, return Err early.
    fn validate_id(data: Option<(&&str, &&str)>) -> Result<String> {
        let (k, v) = Passport::get_kv(data)?;

        let re = regex::Regex::new(r"^(\d){9}$").unwrap();
        if re.is_match(v) {
            return Ok(v.to_owned());
        } else {
            bail!("Invalid: {}", k);
        }
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
