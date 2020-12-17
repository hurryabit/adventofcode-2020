use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day13.txt")?);
    let mut lines = reader.lines();
    let now: i64 = lines.next().unwrap()?.parse()?;
    let ids = lines
        .next()
        .unwrap()?
        .split(',')
        .filter_map(|id| if id == "x" { None } else { Some(id.parse()) })
        .collect::<Result<Vec<i64>, _>>()?;
    let (time, id) = ids
        .iter()
        .map(|id| {
            let next = (now / id + 1) * id;
            (next, id)
        })
        .min_by_key(|x| x.0)
        .unwrap();

    println!("The number is {}", (time - now) * id);
    Ok(())
}
