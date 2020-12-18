use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

type Point = (i64, i64, i64);

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day17.txt")?);
    let mut active: HashSet<Point> = HashSet::new();
    for (x, line) in reader.lines().enumerate() {
        let line = line?;
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                active.insert((x as i64, y as i64, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut neighbors: HashMap<Point, usize> = HashMap::new();
        for (x0, y0, z0) in active.iter() {
            for x in x0-1..=x0+1 {
                for y in y0-1..=y0+1 {
                    for z in z0-1..=z0+1 {
                        if x != *x0 || y != *y0 || z != *z0 {
                            *neighbors.entry((x, y, z)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        active = neighbors.into_iter().filter_map(|(point, num_neighbors)| {
            if num_neighbors == 3 || num_neighbors == 2 && active.contains(&point) {
                Some(point)
            } else {
                None
            }
        }).collect();
    }

    println!("{} cubes are active after 6 cycles", active.len());
    Ok(())
}
