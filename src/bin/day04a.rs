use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let required: BTreeSet<String> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|x| x.to_string()).collect();
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
                if let Some(key) = pair.split(':').next() {
                    current.insert(key.to_owned());
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
