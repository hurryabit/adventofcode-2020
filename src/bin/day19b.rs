use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

type Var = u16;

#[derive(Clone)]
enum Alternative {
    Terminal(char),
    NonTerminals(Vec<Var>),
}

type SurfaceGrammar = HashMap<Var, Vec<Alternative>>;

struct RuntimeGrammar {
    terminal_sets: HashMap<char, HashSet<Var>>,
    non_terminal_sets: HashMap<(Var, Var), HashSet<Var>>,
    alias_sets: HashMap<Var, HashSet<Var>>,
}

use Alternative::*;

fn as_runtime(grammar: &SurfaceGrammar) -> RuntimeGrammar {
    let mut terminal_sets: HashMap<char, HashSet<Var>> = HashMap::new();
    for (var, alts) in grammar {
        for alt in alts {
            if let Terminal(terminal) = alt {
                terminal_sets.entry(*terminal).or_default().insert(*var);
            }
        }
    }

    let mut non_terminal_sets: HashMap<(Var, Var), HashSet<Var>> = HashMap::new();
    let mut alias_sets: HashMap<Var, HashSet<Var>> = HashMap::new();
    for (var, alts) in grammar {
        for alt in alts {
            if let NonTerminals(non_terminals) = alt {
                match non_terminals.len() {
                    1 => {
                        alias_sets.entry(non_terminals[0]).or_default().insert(*var);
                    }
                    2 => {
                        non_terminal_sets
                            .entry((non_terminals[0], non_terminals[1]))
                            .or_default()
                            .insert(*var);
                    }
                    n => panic!("bad lenght of RHS: {}", n),
                }
            }
        }
    }

    // In general it would be necessary to saturate the alias set but the
    // input grammar doesn't have any alias chains.
    RuntimeGrammar { terminal_sets, non_terminal_sets, alias_sets }
}

fn parse(grammar: &RuntimeGrammar, input: &str) -> HashSet<Var> {
    let RuntimeGrammar { terminal_sets, non_terminal_sets, alias_sets } = grammar;
    let mut table: Vec<Vec<HashSet<Var>>> = Vec::new();
    let row0: Vec<_> = input
        .chars()
        .map(|ch| {
            let mut vars = terminal_sets.get(&ch).unwrap().clone();
            #[allow(clippy::redundant_clone)]
            for var in vars.clone().iter() {
                vars.extend(alias_sets.get(var).unwrap_or(&HashSet::new()));
            }
            vars
        })
        .collect();
    let n = row0.len();
    table.push(row0);

    for i in 1..n {
        let rowi = (0..n - i)
            .map(|j| {
                let mut vars = HashSet::new();
                for k in 0..i {
                    for left in &table[k][j] {
                        for right in &table[i - 1 - k][j + 1 + k] {
                            vars.extend(
                                non_terminal_sets.get(&(*left, *right)).unwrap_or(&HashSet::new()),
                            );
                        }
                    }
                }
                #[allow(clippy::redundant_clone)]
                for var in vars.clone().iter() {
                    vars.extend(alias_sets.get(var).unwrap_or(&HashSet::new()));
                }
                vars
            })
            .collect();
        table.push(rowi);
    }
    std::mem::take(&mut table[n - 1][0])
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day19.txt")?);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut grammar = SurfaceGrammar::new();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let colon = line.find(':').unwrap();
        let lhs = line[..colon].parse().unwrap();
        let mut alts = Vec::new();
        let rhs = &line[colon + 2..];
        if let Some(rest) = rhs.strip_prefix('\"') {
            alts.push(Terminal(rest.chars().next().unwrap()));
        } else {
            alts.extend(rhs.split(" | ").map(|non_terminals| {
                NonTerminals(
                    non_terminals
                        .split_ascii_whitespace()
                        .map(|var| var.parse().unwrap())
                        .collect(),
                )
            }));
        }
        grammar.insert(lhs, alts);
    }
    grammar.insert(8, vec![NonTerminals(vec![42]), NonTerminals(vec![42, 8])]);
    grammar.insert(11, vec![NonTerminals(vec![42, 31]), NonTerminals(vec![42, 1000])]);
    grammar.insert(1000, vec![NonTerminals(vec![11, 31])]);

    let grammar = as_runtime(&grammar);

    let mut num_matches: usize = 0;
    for line in lines {
        let vars = parse(&grammar, &line);
        if vars.contains(&0) {
            num_matches += 1;
        }
    }

    println!("{} inputs match rule 0", num_matches);
    Ok(())
}
