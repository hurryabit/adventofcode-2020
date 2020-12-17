use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

type Zi = num_complex::Complex<i64>;

const NORTH: Zi = Zi::new(0, 1);
const SOUTH: Zi = Zi::new(0, -1);
const EAST: Zi = Zi::new(1, 0);
const WEST: Zi = Zi::new(-1, 0);
const LEFT: Zi = NORTH;
const RIGHT: Zi = SOUTH;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day12.txt")?);
    let mut pos = Zi::new(0, 0);
    let mut dir = Zi::new(1, 0); // Facing east.

    for line in reader.lines() {
        let line = line?;
        let num: i64 = line[1..].parse()?;
        match line.chars().next().unwrap() {
            'N' => pos += NORTH.scale(num),
            'S' => pos += SOUTH.scale(num),
            'E' => pos += EAST.scale(num),
            'W' => pos += WEST.scale(num),
            'L' => dir *= LEFT.powi((num / 90) as i32),
            'R' => dir *= RIGHT.powi((num / 90) as i32),
            'F' => pos += dir.scale(num),
            ch => panic!("unexpected char: {}", ch),
        }
    }

    println!("The Manhattan distance is {}", pos.l1_norm());
    Ok(())
}
