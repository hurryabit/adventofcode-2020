use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day03.txt")?);
    let forest: Vec<_> = reader
        .lines()
        .map(|line| {
            let line: Vec<bool> = line.unwrap().chars().map(|c| c == '#').collect();
            line
        })
        .collect();
    let mut col = 0;
    let mut count = 0;
    for line in forest.iter() {
        if line[col] {
            count += 1;
        }
        col = (col + 3) % line.len();
    }
    println!("We'd hit {} trees", count);
    Ok(())
}
