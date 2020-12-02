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
            let pos1: usize = caps.get(1)?.as_str().parse().ok()?;
            let pos2: usize = caps.get(2)?.as_str().parse().ok()?;
            let ch: char = caps.get(3)?.as_str().parse().ok()?;
            let password = caps.get(4)?.as_str();
            let ch1 = password.as_bytes()[pos1 - 1] as char;
            let ch2 = password.as_bytes()[pos2 - 1] as char;
            if (ch1 == ch) != (ch2 == ch) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("Number of valid passwords: {}", num_valid_passwords);
    Ok(())
}
