#![allow(clippy::many_single_char_names)]
use num_integer::Integer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day13.txt")?);
    let mut lines = reader.lines().skip(1);
    let (t, n) =
        lines.next().unwrap()?.split(',').enumerate().fold((0, 1), |(a, p), (b_neg, q)| {
            if q == "x" {
                (a, p)
            } else {
                let b = -(b_neg as i128);
                let q = q.parse().unwrap();
                let e = i128::extended_gcd(&p, &q);
                assert_eq!(e.gcd, 1);
                let r = p * q;
                let c = (b * e.x * p + a * e.y * q) % r;
                (c, r)
            }
        });

    println!("The earliest timestamp is {}", (t + n) % n);
    Ok(())
}
