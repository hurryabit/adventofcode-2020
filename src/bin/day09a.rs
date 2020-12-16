use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day09.txt")?);
    let mut buffer: VecDeque<i64> = VecDeque::new();
    for line in reader.lines() {
        let line = line?;
        let n = line.parse()?;
        if buffer.len() >= 25 {
            if !buffer.iter().any(|p| buffer.iter().any(|q| p != q && n == p + q)) {
                println!("The first invalid number is {}", n);
                return Ok(())
            }
            buffer.pop_front();
        }
        buffer.push_back(n);
    }
    println!("No invalid number found");
    Ok(())
}
