use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct State(HashMap<String, HashSet<String>>);

impl State {
    fn new() -> Self {
        State(HashMap::new())
    }

    fn add_candidates(&mut self, allergen: &str, ingredients: &[&str]) {
        let ingredients = HashSet::from_iter(ingredients.iter().map(|s| String::from(*s)));
        if let Some(candidates) = self.0.get_mut(allergen) {
            candidates.retain(|ingredient| ingredients.contains(ingredient));
        } else {
            self.0.insert(String::from(allergen), ingredients);
        }
    }

    fn allergenic_ingredients(mut self) -> Vec<(String, String)> {
        let mut queue = VecDeque::new();
        for (allergen, ingredients) in self.0.iter() {
            if ingredients.len() == 1 {
                queue.push_back(allergen.clone());
            }
        }
        let mut result = Vec::new();
        while let Some(allergen) = queue.pop_front() {
            let ingredient = self.0.get(&allergen).unwrap().iter().next().unwrap().clone();
            result.push((allergen.clone(), ingredient.clone()));
            for (other_allergen, ingredients) in self.0.iter_mut() {
                if other_allergen == &allergen {
                    continue;
                }
                if ingredients.remove(&ingredient) && ingredients.len() == 1 {
                    queue.push_back(other_allergen.clone());
                }
            }
        }
        result
    }
}

type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

fn parse_input(reader: impl BufRead) -> Result<State> {
    let mut state = State::new();
    for line in reader.lines() {
        let line = line?;
        let mut ingredients = Vec::new();
        let mut seen_contains = false;
        for word in line.trim().split_whitespace() {
            if word == "(contains" {
                seen_contains = true;
            } else if !seen_contains {
                ingredients.push(word);
            } else {
                state.add_candidates(&word[..word.len() - 1], &ingredients);
            }
        }
    }
    Ok(state)
}

fn solve(reader: impl BufRead) -> Result<Vec<String>> {
    let state = parse_input(reader)?;
    let mut allergenic_ingredients = state.allergenic_ingredients();
    allergenic_ingredients.sort_by(|x, y| x.0.cmp(&y.0));
    Ok(allergenic_ingredients.into_iter().map(|x| x.1).collect())
}

fn main() -> Result<()> {
    let file = File::open("inputs/day21.txt")?;
    let reader = BufReader::new(file);
    let result = solve(reader)?;
    println!("The canonical dangerous ingredients list is {}", result.join(","));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "#;
        let result = solve(input.as_bytes()).unwrap();
        assert_eq!(result, vec!["mxmxvkd", "sqjhc", "fvjkl"]);
    }
}
