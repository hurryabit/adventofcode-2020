use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day07.txt")?);
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let mut groups = words.chunks(4);
        let group0 = groups.next().unwrap();
        let target = format!("{} {}", group0[0], group0[1]);
        for group in groups {
            let source = format!("{} {}", group[1], group[2]);
            edges.entry(source).or_default().push(target.clone());
        }
    }

    let mut seen: HashSet<String> = HashSet::new();
    let mut queue = vec![String::from("shiny gold")];
    let empty = Vec::new();
    while let Some(source) = queue.pop() {
        for target in edges.get(&source).unwrap_or(&empty) {
            if !seen.contains(target) {
                seen.insert(target.clone());
                queue.push(target.clone());
            }
        }
    }

    println!("{} outer bag colours are possible", seen.len());
    Ok(())
}
