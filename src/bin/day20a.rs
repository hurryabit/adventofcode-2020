use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Index;

type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

lazy_static::lazy_static! {
    static ref FLIPPED: [u16; 1024] = {
        let mut result = [0; 1024];
        for n in 0..1024 {
            let mut flipped = 0;
            for i in 0..10 {
                flipped |= ((n >> i) & 1) << (9 - i);
            }
            result[n as usize] = flipped;
        }
        result
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TileId(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Border(u16);

impl Border {
    fn flip(self) -> Self {
        Self(FLIPPED[self.0 as usize])
    }

    fn all() -> impl Iterator<Item = Self> {
        (0..1024).map(Self)
    }
}

impl Border {
    fn from_image(chars: impl Iterator<Item = char>) -> Self {
        let mut result = 0;
        for char in chars {
            result <<= 1;
            if char == '#' {
                result |= 1;
            }
        }
        Self(result)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Top = 0,
    Bottom,
    Left,
    Right,
}

type Constraints = [(Dir, Border)];

#[derive(Clone, Copy, Debug)]
struct Tile([Border; 4]);

impl Tile {
    fn from_image(data: Vec<String>) -> Self {
        let top = Border::from_image(data.first().unwrap().chars());
        let bottom = Border::from_image(data.last().unwrap().chars());
        let left = Border::from_image(data.iter().map(|row| row.chars().next().unwrap()));
        let right = Border::from_image(data.iter().map(|row| row.chars().last().unwrap()));
        Self([top, bottom, left, right])
    }
}

impl Index<Dir> for Tile {
    type Output = Border;

    fn index(&self, dir: Dir) -> &Self::Output {
        &self.0[dir as usize]
    }
}

impl Tile {
    fn matches(&self, constraints: &Constraints) -> bool {
        constraints.iter().all(|(dir, border)| self[*dir] == *border)
    }

    fn rotate(&self) -> Self {
        let Self([top, bottom, left, right]) = *self;
        Self([left.flip(), right.flip(), bottom, top])
    }

    fn flip(&self) -> Self {
        let Self([top, bottom, left, right]) = *self;
        Self([left, right, top, bottom])
    }

    fn variants(&self) -> impl Iterator<Item = Self> {
        let mut result = Vec::with_capacity(8);
        let mut tile = *self;
        for _ in 0..4 {
            result.push(tile);
            result.push(tile.flip());
            tile = tile.rotate();
        }
        result.into_iter()
    }

    fn borders(&self) -> impl Iterator<Item = Border> {
        self.0.into_iter()
    }
}

type Solution = Vec<Vec<(TileId, Tile)>>;

struct Puzzle {
    pub size: usize,
    pub tiles: HashMap<TileId, Tile>,
    pub borders: Vec<HashSet<TileId>>,
}

impl Puzzle {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        let mut lines = reader.lines();
        let mut tiles = HashMap::new();
        while let Some(header) = lines.next() {
            let id = TileId(header?[5..9].parse::<u16>()?);
            let mut data = Vec::new();
            for _ in 0..10 {
                data.push(lines.next().unwrap()?);
            }
            tiles.insert(id, Tile::from_image(data));
            assert_eq!(lines.next().unwrap()?, "");
        }
        let size = f64::sqrt(tiles.len() as f64) as usize;
        assert_eq!(tiles.len(), size * size);

        let mut borders = Vec::new();
        borders.resize_with(1024, HashSet::new);
        for (&id, tile) in &tiles {
            for border in tile.borders() {
                borders[border.0 as usize].insert(id);
                borders[border.flip().0 as usize].insert(id);
            }
        }

        Ok(Self { size, tiles, borders })
    }

    fn tiles_with_border(&self, border: Border) -> &HashSet<TileId> {
        &self.borders[border.0 as usize]
    }

    fn find_tile(&self, constraints: &Constraints) -> impl Iterator<Item = (TileId, Tile)> {
        assert!(!constraints.is_empty());
        let mut result = Vec::new();
        let border = constraints[0].1;
        for &id in &self.borders[border.0 as usize] {
            if let Some(tile) = self.tiles[&id].variants().find(|tile| tile.matches(constraints)) {
                result.push((id, tile));
            }
        }
        result.into_iter()
    }

    fn solve_from(&self, id: TileId, tile: Tile) -> Option<Solution> {
        let mut solution: Solution = Vec::new();
        solution.resize_with(self.size, || Vec::with_capacity(self.size));
        solution[0].push((id, tile));
        let mut used = HashSet::from([id]);

        for row in 0..self.size {
            for col in 0..self.size {
                if row == 0 && col == 0 {
                    continue;
                }

                let constraints = if row == 0 {
                    vec![(Dir::Left, solution[row][col - 1].1[Dir::Right])]
                } else if col == 0 {
                    vec![(Dir::Top, solution[row - 1][col].1[Dir::Bottom])]
                } else {
                    vec![
                        (Dir::Left, solution[row][col - 1].1[Dir::Right]),
                        (Dir::Top, solution[row - 1][col].1[Dir::Bottom]),
                    ]
                };
                let candidates: Vec<_> =
                    self.find_tile(&constraints).filter(|(id, _)| !used.contains(id)).collect();
                if candidates.is_empty() {
                    return None;
                } else if candidates.len() >= 2 {
                    panic!("too many candidates");
                } else {
                    let (id, tile) = candidates[0];
                    solution[row].push((id, tile));
                    used.insert(id);
                }
            }
        }
        Some(solution)
    }

    fn solve(&self) -> Option<Solution> {
        for (&id, tile) in &self.tiles {
            let mut tile = *tile;
            for _ in 0..4 {
                if let Some(solution) = self.solve_from(id, tile) {
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
    println!("BORDERS:");
    let mut total = 0;
    for border in Border::all() {
        let ids = puzzle.tiles_with_border(border);
        if !ids.is_empty() {
            println!("{:?} -> {:?}", border, ids);
        }
        total += ids.len();
    }
    println!("total {}", total);

    let solution = puzzle.solve().unwrap();
    let corners =
        [solution[0][0].0 .0, solution[0][11].0 .0, solution[11][0].0 .0, solution[11][11].0 .0];
    let result: u64 = corners.into_iter().map(|x| x as u64).product();
    println!("The product of the IDs of the corner tiles is {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let puzzle = Puzzle::from_reader(EXAMPLE.as_bytes()).unwrap();
        assert_eq!(puzzle.tiles.len(), 9);
        println!("TILES:");
        for (id, tile) in &puzzle.tiles {
            println!("{} -> {:?}", id.0, tile);
        }
        println!("BORDERS:");
        for border in Border::all() {
            let ids = puzzle.tiles_with_border(border);
            if !ids.is_empty() {
                println!("{:?} -> {:?}", border, ids);
            }
        }

        let solution = puzzle.solve().unwrap();
        let mut corners =
            [solution[0][0].0 .0, solution[0][2].0 .0, solution[2][0].0 .0, solution[2][2].0 .0];
        corners.sort_unstable();
        assert_eq!(corners, [1171, 1951, 2971, 3079])
    }

    #[test]
    fn flip_involution() {
        for n in 0..1024 {
            let b = Border(n);
            assert_eq!(b.flip().flip(), b);
        }
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
