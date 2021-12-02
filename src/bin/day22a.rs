use std::collections::VecDeque;

const PLAYER1: [u8; 25] = [
    24, 22, 26, 6, 14, 19, 27, 17, 39, 34, 40, 41, 23, 30, 36, 11, 28, 3, 10, 21, 9, 50, 32, 25, 8,
];

const PLAYER2: [u8; 25] =
    [48, 49, 47, 15, 42, 44, 5, 4, 13, 7, 20, 43, 12, 37, 29, 18, 45, 16, 1, 46, 38, 35, 2, 33, 31];

fn main() {
    let mut player1 = VecDeque::from(PLAYER1);
    let mut player2 = VecDeque::from(PLAYER2);

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.extend([card1, card2]);
        } else {
            player2.extend([card2, card1]);
        }
    }
    let winner = if player1.is_empty() { player2 } else { player1 };
    let score: usize =
        winner.iter().rev().enumerate().map(|(index, value)| (index + 1) * *value as usize).sum();
    println!("The winning player's score is {}", score);
}
