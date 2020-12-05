use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn get_seat_id(boarding_pass: String) -> u16 {
    let mut boarding_pass = boarding_pass.into_bytes();
    for ch in boarding_pass.iter_mut() {
        if *ch == b'F' || *ch == b'L' {
            *ch = b'0';
        } else if *ch == b'B' || *ch == b'R' {
            *ch = b'1';
        } else {
            panic!("Invalid character in boarding pass: {}", ch);
        }
    }
    let boarding_pass = String::from_utf8(boarding_pass).unwrap();
    u16::from_str_radix(&boarding_pass, 2).unwrap()
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day05.txt")?);
    let mut seat_ids: Vec<_> = reader.lines().map(|line| get_seat_id(line.unwrap())).collect();
    seat_ids.sort_unstable();
    for (s, t) in seat_ids.iter().zip(seat_ids.iter().skip(1)) {
        if s + 1 != *t {
            println!("The seat ID is {}", s + 1);
            return Ok(())
        }
    }
    println!("No seat ID found");
    Ok(())
}
