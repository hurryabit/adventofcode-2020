use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day11.txt")?);
    let mut board = reader
        .lines()
        .map(|line| line.map(|line| line.bytes().collect()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?;
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        let new_board = board
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let i_min = if i == 0 { 0 } else { i - 1 };
                let i_max = std::cmp::min(i + 1, board.len() - 1);
                line.iter()
                    .enumerate()
                    .map(|(j, mut ch)| {
                        let j_min = if j == 0 { 0 } else { j - 1 };
                        let j_max = std::cmp::min(j + 1, board[0].len() - 1);
                        let occupied_neighbors = (i_min..=i_max)
                            .flat_map(|i0| (j_min..=j_max).map(move |j0| (i0, j0)))
                            .filter(|(i0, j0)| (*i0 != i || *j0 != j) && board[*i0][*j0] == b'#')
                            .count();
                        if *ch == b'L' && occupied_neighbors == 0 {
                            ch = &b'#';
                            has_changed = true;
                        } else if *ch == b'#' && occupied_neighbors >= 4 {
                            ch = &b'L';
                            has_changed = true;
                        }
                        *ch
                    })
                    .collect()
            })
            .collect();
        board = new_board;
    }

    let occupied_seats =
        board.iter().flat_map(|line| line.iter()).filter(|&ch| *ch == b'#').count();
    println!("{} seats end up occupied", occupied_seats);
    Ok(())
}
