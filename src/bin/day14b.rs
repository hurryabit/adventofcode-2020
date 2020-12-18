use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day14.txt")?);
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut or_mask = 0;
    let mut floating = vec![];
    for line in reader.lines() {
        let line = line?;

        match &line[..4] {
            "mask" => {
                let mask = &line[7..];
                or_mask = u64::from_str_radix(&mask.replace('X', "0"), 2)?;
                floating = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, ch)| if ch == 'X' { Some(i) } else { None })
                    .collect();
            }
            "mem[" => {
                let bracket = line.find(']').unwrap();
                let value: u64 = line[bracket + 4..].parse()?;
                let mut addr: u64 = line[4..bracket].parse()?;
                addr |= or_mask;
                for bits in 0..(1 << floating.len()) {
                    for (bit_pos, float_pos) in floating.iter().enumerate() {
                        if bits & (1 << bit_pos) == 0 {
                            addr &= !(1 << float_pos);
                        } else {
                            addr |= 1 << float_pos;
                        }
                    }
                    mem.insert(addr, value);
                }
            }
            _ => panic!("unknown instruction: {}", line),
        }
    }
    let sum: u64 = mem.values().sum();
    println!("The sum of values in memory is {}", sum);
    Ok(())
}
