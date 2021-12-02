use std::collections::{HashSet, VecDeque};

const DECK1: [u8; 25] = [
    24, 22, 26, 6, 14, 19, 27, 17, 39, 34, 40, 41, 23, 30, 36, 11, 28, 3, 10, 21, 9, 50, 32, 25, 8,
];

const DECK2: [u8; 25] =
    [48, 49, 47, 15, 42, 44, 5, 4, 13, 7, 20, 43, 12, 37, 29, 18, 45, 16, 1, 46, 38, 35, 2, 33, 31];

#[derive(Clone, Copy, Debug)]
enum Player {
    One,
    Two,
}

type Deck = VecDeque<u8>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Config {
    deck1: Deck,
    deck2: Deck,
}

fn find_winner(mut config: Config) -> (Player, Config) {
    let mut seen: HashSet<Config> = HashSet::new();
    let winner = loop {
        if seen.contains(&config) {
            break Player::One;
        }
        seen.insert(config.clone());

        match (config.deck1.front(), config.deck2.front()) {
            (None, None) => panic!("at least one player should have a card"),
            (None, Some(_)) => break Player::Two,
            (Some(_), None) => break Player::One,
            (Some(&card1), Some(&card2)) => {
                config.deck1.pop_front();
                config.deck2.pop_front();
                let winner = if card1 as usize <= config.deck1.len()
                    && card2 as usize <= config.deck2.len()
                {
                    let (winner, _) = find_winner(Config {
                        deck1: config.deck1.iter().take(card1 as usize).cloned().collect(),
                        deck2: config.deck2.iter().take(card2 as usize).cloned().collect(),
                    });
                    winner
                } else if card1 > card2 {
                    Player::One
                } else {
                    Player::Two
                };
                match winner {
                    Player::One => config.deck1.extend([card1, card2]),
                    Player::Two => config.deck2.extend([card2, card1]),
                }
            }
        }
    };
    (winner, config)
}

fn solve<const M: usize, const N: usize>(deck1: [u8; M], deck2: [u8; N]) -> usize {
    let config = Config { deck1: VecDeque::from(deck1), deck2: VecDeque::from(deck2) };
    let (winner, config) = find_winner(config);
    let deck = match winner {
        Player::One => config.deck1,
        Player::Two => config.deck2,
    };
    deck.iter().rev().enumerate().map(|(index, value)| (index + 1) * *value as usize).sum()
}

fn main() {
    let score = solve(DECK1, DECK2);
    println!("The winning player's score is {}", score);
}

#[test]
fn example1() {
    let deck1 = [9, 2, 6, 3, 1];
    let deck2 = [5, 8, 4, 7, 10];
    let score = solve(deck1, deck2);
    assert_eq!(score, 291);
}

#[test]
fn example2() {
    assert_eq!(solve([43, 19], [2, 29, 14]), 105);
}
