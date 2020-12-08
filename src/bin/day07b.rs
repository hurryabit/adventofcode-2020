use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day07.txt")?);
    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let mut groups = words.chunks(4);
        let group0 = groups.next().unwrap();
        let outer = format!("{} {}", group0[0], group0[1]);
        let inners = if words[4] == "no" && words[5] == "other" {
            Vec::new()
        } else {
            groups
                .map(|group| (group[0].parse().unwrap(), format!("{} {}", group[1], group[2])))
                .collect()
        };
        rules.insert(outer, inners);
    }

    let start = "shiny gold";
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut queue = vec![(start.to_owned(), true)];
    while let Some((outer, down)) = queue.pop() {
        let inners = rules.get(&outer).unwrap();
        if down {
            queue.push((outer, false));
            queue.extend(inners.iter().map(|(_, inner)| (inner.clone(), true)));
        } else {
            let inners_size: usize =
                inners.iter().map(|(count, inner)| count * sizes.get(inner).unwrap()).sum();
            sizes.insert(outer, inners_size + 1);
        }
    }
    let size = sizes.get(start).unwrap();

    println!("We need {} in our {} bag", size - 1, start);
    Ok(())
}
