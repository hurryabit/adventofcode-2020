use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day16.txt")?);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut fields = HashMap::new();
    let mut allowed = vec![false; 1000];
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let colon = line.find(':').unwrap();
        let words: Vec<_> = line[colon + 2..].split(&[' ', '-'][..]).collect();
        assert_eq!(words[2], "or");
        let range1 = words[0].parse().unwrap()..=words[1].parse().unwrap();
        let range2 = words[3].parse().unwrap()..=words[4].parse().unwrap();
        fields.insert(line[..colon].to_owned(), [range1.clone(), range2.clone()]);
        for allow in &mut allowed[range1] {
            *allow = true;
        }
        for allow in &mut allowed[range2] {
            *allow = true;
        }
    }
    assert_eq!(lines.next().unwrap(), String::from("your ticket:"));
    let my_ticket: Vec<usize> =
        lines.next().unwrap().split(',').map(|number| number.parse().unwrap()).collect();
    assert_eq!(lines.next(), Some(String::new()));
    assert_eq!(lines.next().unwrap(), String::from("nearby tickets:"));
    let mut categories: Vec<HashSet<&String>> = Vec::new();
    categories.resize_with(my_ticket.len(), || fields.keys().collect());

    for line in lines {
        let ticket: Vec<usize> = line.split(',').map(|number| number.parse().unwrap()).collect();
        if ticket.iter().all(|number| allowed[*number]) {
            for (number, categories) in ticket.iter().zip(categories.iter_mut()) {
                *categories = categories
                    .iter()
                    .filter_map(|&category| {
                        let [range1, range2] = fields.get(category).unwrap();
                        if range1.contains(number) || range2.contains(number) {
                            Some(category)
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }
    }

    let mut uniques: VecDeque<&String> = categories
        .iter()
        .filter_map(|categories| {
            if categories.len() == 1 {
                Some(*categories.iter().next().unwrap())
            } else {
                None
            }
        })
        .collect();

    while let Some(unique) = uniques.pop_front() {
        for categories in &mut categories {
            if categories.len() > 1 {
                categories.remove(unique);
                if categories.len() == 1 {
                    uniques.push_back(*categories.iter().next().unwrap());
                }
            }
        }
    }

    let product: usize = categories
        .into_iter()
        .zip(my_ticket.iter())
        .filter_map(|(categories, number)| {
            assert_eq!(categories.len(), 1);
            let category = *categories.iter().next().unwrap();
            if category.starts_with("departure") {
                Some(number)
            } else {
                None
            }
        })
        .product();
    println!("The product of all departure fields is {}", product);

    Ok(())
}
