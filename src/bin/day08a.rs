use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day08.txt")?);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut acc: i64 = 0;
    let mut ctr: i64 = 0;
    let mut seen = BTreeSet::new();

    loop {
        if seen.contains(&ctr) {
            break;
        } else {
            seen.insert(ctr);
        }
        let words: Vec<_> = lines[ctr as usize].split_ascii_whitespace().collect();
        match words[0] {
            "nop" => ctr += 1,
            "acc" => {
                acc += words[1].parse::<i64>()?;
                ctr += 1;
            }
            "jmp" => ctr += words[1].parse::<i64>()?,
            other => panic!("unknown instruction: {}", other),
        }
    }

    println!("acc = {}", acc);
    Ok(())
}
