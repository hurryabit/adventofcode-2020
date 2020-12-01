use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day01.txt")?);
    let mut numbers =
        reader.lines().map(|line| Ok(line?.parse()?)).collect::<Result<Vec<u32>, Error>>()?;
    numbers.sort_unstable();
    for (i, x) in numbers.iter().enumerate().take_while(|x| 3 * x.1 < 2020) {
        for (j, y) in numbers.iter().enumerate().skip(i + 1).take_while(|y| x + 2 * y.1 < 2020) {
            let z = 2020 - (x + y);
            if numbers[j + 1..].binary_search(&z).is_ok() {
                println!("The solution is {}", x * y * z);
                return Ok(());
            }
        }
    }
    panic!("No solution found");
}
