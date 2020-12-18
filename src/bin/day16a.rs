use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day16.txt")?);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut allowed = vec![false; 1000];
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let colon = line.find(':').unwrap();
        let words: Vec<_> = line[colon + 2..].split(&[' ', '-'][..]).collect();
        assert_eq!(words[2], "or");
        for allow in &mut allowed[words[0].parse().unwrap()..=words[1].parse().unwrap()] {
            *allow = true;
        }
        for allow in &mut allowed[words[3].parse().unwrap()..=words[4].parse().unwrap()] {
            *allow = true;
        }
    }
    lines.by_ref().take_while(|line| !line.is_empty()).for_each(std::mem::drop);
    assert_eq!(lines.next(), Some(String::from("nearby tickets:")));

    let mut error_rate = 0;
    for line in lines {
        for number in line.split(',') {
            let number: usize = number.parse().unwrap();
            if !allowed[number] {
                error_rate += number;
            }
        }
    }

    println!("The error rate is {}", error_rate);
    Ok(())
}
