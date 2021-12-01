use std::collections::{HashMap, HashSet};
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

    fn allergenic_ingredients(self) -> HashSet<String> {
        let mut result = HashSet::new();
        for ingredients in self.0.into_values() {
            result.extend(ingredients);
        }
        result
    }
}

type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

fn parse_input(reader: impl BufRead) -> Result<(Vec<String>, State)> {
    let mut all_ingredients = Vec::new();
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
                all_ingredients.push(String::from(word));
            } else {
                state.add_candidates(&word[..word.len() - 1], &ingredients);
            }
        }
    }
    Ok((all_ingredients, state))
}

fn solve(reader: impl BufRead) -> Result<usize> {
    let (all_ingredients, state) = parse_input(reader)?;
    let allergenic_ingredients = state.allergenic_ingredients();
    let result = all_ingredients
        .iter()
        .filter(|ingredient| !allergenic_ingredients.contains(*ingredient))
        .count();
    Ok(result)
}

fn main() -> Result<()> {
    let file = File::open("inputs/day21.txt")?;
    let reader = BufReader::new(file);
    let result = solve(reader)?;
    println!("{} occurences of ingredients cannot possibly contain allergenes", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut state = State::new();
        let ingredients1 = &["mxmxvkd", "kfcds", "sqjhc", "nhms"];
        state.add_candidates("dairy", ingredients1);
        state.add_candidates("fish", ingredients1);
        let ingredients2 = &["trh", "fvjkl", "sbzzf", "mxmxvkd"];
        state.add_candidates("dairy", ingredients2);
        let ingredients3 = &["sqjhc", "fvjkl"];
        state.add_candidates("soy", ingredients3);
        let ingredients4 = &["sqjhc", "mxmxvkd", "sbzzf"];
        state.add_candidates("fish", ingredients4);

        let expected =
            HashSet::from_iter(["mxmxvkd", "sqjhc", "fvjkl"].iter().map(|s| String::from(*s)));

        assert_eq!(state.allergenic_ingredients(), expected);
    }

    #[test]
    fn test_2() {
        let input = r#"
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "#;
        let (_, state) = parse_input(input.as_bytes()).unwrap();

        let expected =
            HashSet::from_iter(["mxmxvkd", "sqjhc", "fvjkl"].iter().map(|s| String::from(*s)));
        assert_eq!(state.allergenic_ingredients(), expected);
    }

    #[test]
    fn test_3() {
        let input = r#"
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "#;
        let result = solve(input.as_bytes()).unwrap();
        assert_eq!(result, 5);
    }
}
