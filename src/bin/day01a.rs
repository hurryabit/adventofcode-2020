use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day01.txt")?);
    let numbers =
        reader.lines().map(|line| Ok(line?.parse()?)).collect::<Result<Vec<u32>, Error>>()?;
    let set1: BTreeSet<_> = numbers.iter().copied().collect();
    let set2: BTreeSet<_> = numbers.into_iter().map(|n| 2020 - n).collect();
    let n = set1.intersection(&set2).next().expect("No result found");
    println!("The result is {}", (2020 - n) * n);
    Ok(())
}
