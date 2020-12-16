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

    let mut counts: Vec<u64> = Vec::new();
    counts.resize(adapters.len(), 0);
    let mut i = counts.len() - 1;
    counts[i] = 1;
    while i > 0 {
        i -= 1;
        let adapter = adapters[i];
        let mut count = 0;
        let mut j = i + 1;
        while j < counts.len() && adapters[j] - adapter <= 3 {
            count += counts[j];
            j += 1;
        }
        counts[i] = count;
    }

    println!("The adapters can be arranged in {} ways", counts[0]);
    Ok(())
}
