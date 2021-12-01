use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

const FIRST_INVALID: i64 = 375054920;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day09.txt")?);
    let numbers = reader
        .lines()
        .map(|line| -> Result<i64, Error> { Ok(line?.parse()?) })
        .collect::<Result<Vec<_>, _>>()?;
    let mut prefix_sums = vec![0];
    prefix_sums.extend(numbers.iter().scan(0, |sum, number| {
        *sum += number;
        Some(*sum)
    }));
    let mut left = 0;
    let mut right = 0;

    loop {
        if right >= prefix_sums.len() {
            panic!("No encryption weakness found");
        }
        match (prefix_sums[left] + FIRST_INVALID).cmp(&prefix_sums[right]) {
            Ordering::Less => left += 1,
            Ordering::Equal => break,
            Ordering::Greater => right += 1,
        }
    }
    let segment = &numbers[left - 1..right - 1];
    let weakness = segment.iter().min().unwrap() + segment.iter().max().unwrap();
    println!("The encryption weakness is {}", weakness);
    Ok(())
}
