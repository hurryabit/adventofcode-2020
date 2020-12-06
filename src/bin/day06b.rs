use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day06.txt")?);
    let mut count = 0;
    let mut seen = BTreeSet::new();
    let mut first = true;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            count += seen.len();
            first = true;
        } else if first {
            seen = line.chars().collect();
            first = false;
        } else {
            seen = seen.intersection(&line.chars().collect()).copied().collect();
        }
    }
    count += seen.len();
    println!("The sum of the counts is {}", count);
    Ok(())
}
