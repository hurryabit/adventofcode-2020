use std::collections::HashMap;

fn main() {
    let input = &[6,13,1,15,2,0];
    let len = input.len();
    let mut history: HashMap<usize, usize> =
        input[..len - 1].iter().enumerate().map(|(i, n)| (*n, i + 1)).collect();
    let mut number = input[len - 1];
    for turn in len + 1..=30000000 {
        let prev = history.get(&number).copied();
        history.insert(number, turn - 1);
        number = prev.map_or(0, |prev| turn - 1 - prev);
    }
    println!("The 30,000,000th number is {}", number);
}
