use regex::Regex;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let required: BTreeSet<String> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|x| x.to_string()).collect();
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$")?;
    let ecl_values: BTreeSet<String> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().map(|x| x.to_string()).collect();

    let mut current = BTreeSet::new();
    let mut count = 0;

    let reader = BufReader::new(File::open("inputs/day04.txt")?);
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if required.is_subset(&current) {
                count += 1;
            }
            current.clear();
        } else {
            for pair in line.split_ascii_whitespace() {
                if let Some(colon) = pair.find(':') {
                    let key = &pair[..colon];
                    let value = &pair[colon + 1..];

                    let good = match key {
                        "byr" => value.parse().map_or(false, |byr: u16| (1920..=2002).contains(&byr)),
                        "iyr" => value.parse().map_or(false, |iyr: u16| (2010..=2020).contains(&iyr)),
                        "eyr" => value.parse().map_or(false, |iyr: u16| (2020..=2030).contains(&iyr)),
                        "hgt" => {
                            let number: Result<u16, _> = value[..value.len() - 2].parse();
                            match &value[value.len() - 2..] {
                                "cm" => number.map_or(false, |hgt| (150..=193).contains(&hgt)),
                                "in" => number.map_or(false, |hgt| (59..=76).contains(&hgt)),
                                _ => false,
                            }
                        }
                        "hcl" => hcl_regex.is_match(value),
                        "ecl" => ecl_values.contains(value),
                        "pid" => value.len() == 9 && value.chars().all(|x| x.is_ascii_digit()),
                        _ => false,
                    };
                    if good {
                        current.insert(key.to_string());
                    }
                }
            }
        }
    }
    if required.is_subset(&current) {
        count += 1;
    }
    println!("There are {} valid passports", count);
    Ok(())
}
