use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day02.txt")?);
    let regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")?;
    let num_valid_passwords = reader
        .lines()
        .filter_map(|line| -> Option<()> {
            let line = line.ok()?;
            let caps = regex.captures(&line)?;
            let min: usize = caps.get(1)?.as_str().parse().ok()?;
            let max: usize = caps.get(2)?.as_str().parse().ok()?;
            let ch: char = caps.get(3)?.as_str().parse().ok()?;
            let password = caps.get(4)?.as_str();
            let count = password.chars().filter(|x| *x == ch).count();
            if min <= count && count <= max {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("Number of valid passwords: {}", num_valid_passwords);
    Ok(())
}
