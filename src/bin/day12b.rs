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
    let mut position = Zi::new(0, 0);
    let mut waypoint = Zi::new(10, 1);

    for line in reader.lines() {
        let line = line?;
        let num: i64 = line[1..].parse()?;
        match line.chars().next().unwrap() {
            'N' => waypoint += NORTH.scale(num),
            'S' => waypoint += SOUTH.scale(num),
            'E' => waypoint += EAST.scale(num),
            'W' => waypoint += WEST.scale(num),
            'L' => waypoint *= LEFT.powi((num / 90) as i32),
            'R' => waypoint *= RIGHT.powi((num / 90) as i32),
            'F' => position += waypoint.scale(num),
            ch => panic!("unexpected char: {}", ch),
        }
    }

    println!("The Manhattan distance is {}", position.l1_norm());
    Ok(())
}
