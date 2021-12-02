use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

type Coord = (i64, i64);

pub struct Floor {
    black_tiles: HashSet<Coord>,
}

impl Floor {
    pub fn from_reader(reader: impl BufRead) -> Result<Self> {
        let mut black_tiles = HashSet::new();

        for line in reader.lines() {
            let (mut x, mut y) = (0, 0);
            let line = line?;
            let mut chars = line.trim().chars();
            while let Some(char) = chars.next() {
                match char {
                    'e' => x += 1,
                    'w' => x -= 1,
                    'n' => match chars.next().unwrap() {
                        'e' => y += 1,
                        'w' => {
                            x -= 1;
                            y += 1;
                        }
                        char => panic!("bad direction: n{}", char),
                    },
                    's' => match chars.next().unwrap() {
                        'e' => {
                            x += 1;
                            y -= 1;
                        }
                        'w' => y -= 1,
                        char => panic!("bad direction: s{}", char),
                    },
                    _ => panic!("bad direction: {}", char),
                }
            }
            if black_tiles.contains(&(x, y)) {
                black_tiles.remove(&(x, y));
            } else {
                black_tiles.insert((x, y));
            }
        }

        Ok(Self { black_tiles })
    }

    pub fn num_black_tiles(&self) -> usize {
        self.black_tiles.len()
    }

    pub fn step(&mut self) {
        fn neighbors((x, y): Coord) -> [Coord; 6] {
            [(x + 1, y), (x, y + 1), (x - 1, y + 1), (x, y - 1), (x - 1, y), (x + 1, y - 1)]
        }

        let mut new_black_tiles = HashSet::new();
        for &black_tile in &self.black_tiles {
            let (black_neighbors, white_neighbors): (Vec<_>, Vec<_>) = neighbors(black_tile)
                .into_iter()
                .partition(|neighbor| self.black_tiles.contains(neighbor));
            if !(black_neighbors.is_empty() || black_neighbors.len() > 2) {
                new_black_tiles.insert(black_tile);
            }
            for white_tile in white_neighbors {
                let black_neighbors = neighbors(white_tile)
                    .into_iter()
                    .filter(|neighbor| self.black_tiles.contains(neighbor));
                if black_neighbors.count() == 2 {
                    new_black_tiles.insert(white_tile);
                }
            }
        }

        self.black_tiles = new_black_tiles;
    }

    pub fn multi_step(&mut self, num_steps: usize) {
        for _ in 0..num_steps {
            self.step();
        }
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/day24.txt")?;
    let reader = BufReader::new(file);
    let mut floor = Floor::from_reader(reader)?;
    floor.multi_step(100);
    println!("{} tiles will be black", floor.num_black_tiles());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r##"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"##;

    #[test]
    fn example() {
        let mut floor = Floor::from_reader(EXAMPLE_INPUT.as_bytes()).unwrap();
        assert_eq!(floor.num_black_tiles(), 10);
        floor.multi_step(100);
        assert_eq!(floor.num_black_tiles(), 2208);
    }
}
