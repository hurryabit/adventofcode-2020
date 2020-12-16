use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day10.txt")?);
    let mut adapters = std::iter::once(Ok(0))
        .chain(reader.lines().map(|line| -> Result<i32, Error> { Ok(line?.parse()?) }))
        .collect::<Result<Vec<_>, _>>()?;
    adapters.sort_unstable();
    let mut ones = 0;
    let mut threes = 1; // For the difference between the final adapter and the device.
    for (m, n) in adapters.iter().zip(adapters.iter().skip(1)) {
        match n - m {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }
    println!("The number is {}", ones * threes);
    Ok(())
}
