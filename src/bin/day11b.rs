use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

const DELTAS: &[(isize, isize)] =
    &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

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
            .map(|(i0, line)| {
                let i0 = i0 as isize;
                line.iter()
                    .enumerate()
                    .map(|(j0, mut ch)| {
                        let j0 = j0 as isize;
                        let mut occupied_neighbors = 0;
                        for (di, dj) in DELTAS {
                            let mut i = i0;
                            let mut j = j0;
                            let occupied = loop {
                                i += di;
                                j += dj;
                                if i < 0
                                    || i as usize >= board.len()
                                    || j < 0
                                    || j as usize >= board[0].len()
                                {
                                    break false;
                                } else {
                                    match board[i as usize][j as usize] {
                                        b'L' => break false,
                                        b'#' => break true,
                                        b'.' => continue,
                                        ch => panic!("unexpected character: {}", ch),
                                    }
                                }
                            };
                            if occupied {
                                occupied_neighbors += 1;
                            }
                        }

                        if *ch == b'L' && occupied_neighbors == 0 {
                            ch = &b'#';
                            has_changed = true;
                        } else if *ch == b'#' && occupied_neighbors >= 5 {
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
