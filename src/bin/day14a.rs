use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day14.txt")?);
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = (1 << 36) - 1;
    for line in reader.lines() {
        let line = line?;

        match &line[..4] {
            "mask" => {
                let mask = &line[7..];
                and_mask = u64::from_str_radix(&mask.replace('X', "1"), 2)?;
                or_mask = u64::from_str_radix(&mask.replace('X', "0"), 2)?;
            }
            "mem[" => {
                let bracket = line.find(']').unwrap();
                let addr = line[4..bracket].parse()?;
                let value: u64 = line[bracket+4..].parse()?;
                mem.insert(addr, value & and_mask | or_mask);
            }
            _ => panic!("unknown instruction: {}", line),
        }
    }
    let sum: u64 = mem.values().sum();
    println!("The sum of values in memory is {}", sum);
    Ok(())
}
