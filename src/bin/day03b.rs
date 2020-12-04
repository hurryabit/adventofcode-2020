use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day03.txt")?);
    let forest: Vec<_> = reader
        .lines()
        .map(|line| {
            let line: Vec<bool> = line.unwrap().chars().map(|c| c == '#').collect();
            line
        })
        .collect();
    let width = forest[0].len();
    let result: i64 = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|(d_row, d_col)| {
            let mut row = 0;
            let mut col = 0;
            let mut count = 0;
            while row < forest.len() {
                if forest[row][col] {
                    count += 1;
                }
                row += d_row;
                col = (col + d_col) % width;
            }
            count
        })
        .product();

    println!("The magic number is {}", result);
    Ok(())
}
