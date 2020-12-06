use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day06.txt")?);
    let mut count = 0;
    let mut seen = BTreeSet::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            count += seen.len();
            seen.clear();
        } else {
            seen.extend(line.chars());
        }
    }
    count += seen.len();
    println!("The sum of the counts is {}", count);
    Ok(())
}
