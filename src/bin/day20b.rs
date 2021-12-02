use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const SEAMONSTER: [&str; 3] =
    ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "];

type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TileId(u16);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Border(Vec<u8>);

impl Border {
    fn flip(&self) -> Self {
        let mut result = self.0.clone();
        result.reverse();
        Border(result)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Top = 0,
    Bottom,
    Left,
    Right,
}

impl Dir {
    const ALL: [Self; 4] = [Self::Top, Self::Bottom, Self::Left, Self::Right];
}

type Constraints = [(Dir, Border)];

#[derive(Clone, Debug)]
struct Tile(Vec<Vec<u8>>);

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for row in &self.0 {
            if first {
                first = false;
            } else {
                writeln!(f)?;
            }
            write!(f, "{}", std::str::from_utf8(row).unwrap())?;
        }
        Ok(())
    }
}

impl Tile {
    #![allow(clippy::needless_range_loop)]
    fn seamonster() -> Self {
        Self(SEAMONSTER.iter().map(|row| Vec::from(row.as_bytes())).collect())
    }

    fn border(&self, dir: Dir) -> Border {
        Border(match dir {
            Dir::Top => self.0.first().unwrap().clone(),
            Dir::Bottom => self.0.last().unwrap().clone(),
            Dir::Left => self.0.iter().map(|row| *row.first().unwrap()).collect(),
            Dir::Right => self.0.iter().map(|row| *row.last().unwrap()).collect(),
        })
    }

    fn matches(&self, constraints: &Constraints) -> bool {
        constraints.iter().all(|(dir, border)| self.border(*dir) == *border)
    }

    fn rotate(&self) -> Self {
        let n = self.0.len();
        let mut result = self.0.clone();
        for i in 0..n {
            for j in 0..n {
                result[i][j] = self.0[j][n - 1 - i];
            }
        }
        Self(result)
    }

    fn flip(&self) -> Self {
        let n = self.0.len();
        let mut result = self.0.clone();
        for i in 0..n {
            for j in 0..n {
                result[i][j] = self.0[j][i];
            }
        }
        Self(result)
    }

    fn variants(&self) -> impl Iterator<Item = Self> {
        let mut result = Vec::with_capacity(8);
        let mut tile = self.clone();
        for _ in 0..4 {
            result.push(tile.clone());
            result.push(tile.flip());
            tile = tile.rotate();
        }
        result.into_iter()
    }

    fn peel(&mut self) {
        self.0.remove(0);
        self.0.pop();
        for row in &mut self.0 {
            row.remove(0);
            row.pop();
        }
    }

    fn glue_row(row: &[Self]) -> Self {
        let mut result = Vec::new();
        result.resize_with(row[0].0.len(), Vec::new);
        for tile in row {
            for (result_row, tile_row) in result.iter_mut().zip(&tile.0) {
                result_row.extend(tile_row);
            }
        }
        Self(result)
    }

    fn glue_matrix(matrix: &[Vec<Self>]) -> Self {
        let mut result = Vec::new();
        for row in matrix {
            result.extend(Self::glue_row(row).0);
        }
        Self(result)
    }

    fn matches_subtile(&self, sub: &Tile, row: usize, col: usize) -> bool {
        (0..sub.0.len()).all(|i| {
            (0..sub.0[0].len())
                .all(|j| sub.0[i][j] == b' ' || sub.0[i][j] == self.0[row + i][col + j])
        })
    }

    fn find_subtile(&self, sub: &Tile) -> Vec<(usize, usize)> {
        let self_m = self.0.len();
        let self_n = self.0[0].len();
        let sub_m = sub.0.len();
        let sub_n = sub.0[0].len();
        let mut result = Vec::new();
        for i in 0..=(self_m - sub_m) {
            for j in 0..=(self_n - sub_n) {
                if self.matches_subtile(sub, i, j) {
                    result.push((i, j));
                }
            }
        }
        result
    }

    fn count(&self, pixel: u8) -> usize {
        self.0.iter().flat_map(|row| row.iter()).filter(|x| **x == pixel).count()
    }
}

type Solution = Vec<Vec<Tile>>;

struct Puzzle {
    pub size: usize,
    pub tiles: HashMap<TileId, Tile>,
    pub borders: HashMap<Border, HashSet<TileId>>,
}

impl Puzzle {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        let mut lines = reader.lines();
        let mut tiles = HashMap::new();
        let mut borders = HashMap::new();
        while let Some(header) = lines.next() {
            let id = TileId(header?[5..9].parse::<u16>()?);
            let mut data = Vec::new();
            for _ in 0..10 {
                data.push(lines.next().unwrap()?.into_bytes());
            }
            let tile = Tile(data);
            for dir in Dir::ALL {
                let border = tile.border(dir);
                borders.entry(border.flip()).or_insert_with(HashSet::new).insert(id);
                borders.entry(border).or_insert_with(HashSet::new).insert(id);
            }
            tiles.insert(id, tile.clone());

            assert_eq!(lines.next().unwrap()?, "");
        }
        let size = f64::sqrt(tiles.len() as f64) as usize;
        assert_eq!(tiles.len(), size * size);

        Ok(Self { size, tiles, borders })
    }

    fn find_tile(&self, constraints: &Constraints) -> impl Iterator<Item = (TileId, Tile)> {
        assert!(!constraints.is_empty());
        let mut result = Vec::new();
        let border = constraints[0].1.clone();
        for &id in self.borders.get(&border).unwrap() {
            if let Some(tile) = self.tiles[&id].variants().find(|tile| tile.matches(constraints)) {
                result.push((id, tile));
            }
        }
        result.into_iter()
    }

    fn solve_from(&self, id: TileId, tile: Tile) -> Option<Solution> {
        let mut solution: Solution = Vec::new();
        solution.resize_with(self.size, || Vec::with_capacity(self.size));
        solution[0].push(tile);
        let mut used = HashSet::from([id]);

        for row in 0..self.size {
            for col in 0..self.size {
                if row == 0 && col == 0 {
                    continue;
                }

                let constraints = if row == 0 {
                    vec![(Dir::Left, solution[row][col - 1].border(Dir::Right))]
                } else if col == 0 {
                    vec![(Dir::Top, solution[row - 1][col].border(Dir::Bottom))]
                } else {
                    vec![
                        (Dir::Left, solution[row][col - 1].border(Dir::Right)),
                        (Dir::Top, solution[row - 1][col].border(Dir::Bottom)),
                    ]
                };
                let candidates: Vec<_> =
                    self.find_tile(&constraints).filter(|(id, _)| !used.contains(id)).collect();
                if candidates.is_empty() {
                    return None;
                } else if candidates.len() >= 2 {
                    panic!("too many candidates");
                } else {
                    let (id, tile) = candidates.into_iter().next().unwrap();
                    solution[row].push(tile);
                    used.insert(id);
                }
            }
        }
        Some(solution)
    }

    fn solve(&self) -> Option<Solution> {
        for (&id, tile) in &self.tiles {
            let mut tile = tile.clone();
            for _ in 0..4 {
                if let Some(solution) = self.solve_from(id, tile.clone()) {
                    return Some(solution);
                }
                tile = tile.rotate();
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/day20.txt")?;
    let reader = BufReader::new(file);
    let puzzle = Puzzle::from_reader(reader)?;
    assert_eq!(puzzle.tiles.len(), 144);

    let mut solution = puzzle.solve().unwrap();
    for row in &mut solution {
        for tile in row {
            tile.peel();
        }
    }
    let tile = Tile::glue_matrix(&solution);
    let num_seamonsters = tile
        .variants()
        .find_map(|tile| {
            let n = tile.find_subtile(&Tile::seamonster()).len();
            if n > 0 {
                Some(n)
            } else {
                None
            }
        })
        .unwrap();
    let result = tile.count(b'#') - 15 * num_seamonsters;
    println!("{} `#` are not part of a seam monster", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let puzzle = Puzzle::from_reader(EXAMPLE.as_bytes()).unwrap();
        assert_eq!(puzzle.tiles.len(), 9);

        let mut solution = puzzle.solve().unwrap();
        for row in &mut solution {
            for tile in row {
                tile.peel();
            }
        }
        let tile = Tile::glue_matrix(&solution);
        let num_seamonsters = tile
            .variants()
            .find_map(|tile| {
                let n = tile.find_subtile(&Tile::seamonster()).len();
                if n > 0 {
                    Some(n)
                } else {
                    None
                }
            })
            .unwrap();
        let result = tile.count(b'#') - 15 * num_seamonsters;
        assert_eq!(result, 273);
    }

    const EXAMPLE: &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

"#;
}
