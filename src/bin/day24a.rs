use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Error = Box<dyn std::error::Error>;

type Result<T> = std::result::Result<T, Error>;

fn solve(reader: impl BufRead) -> Result<usize> {
    let mut blacks = HashSet::new();

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
        if blacks.contains(&(x, y)) {
            blacks.remove(&(x, y));
        } else {
            blacks.insert((x, y));
        }
    }

    Ok(blacks.len())
}

fn main() -> Result<()> {
    let file = File::open("inputs/day24.txt")?;
    let reader = BufReader::new(file);
    let result = solve(reader)?;
    println!("{} tiles are left with the black side up", result);
    Ok(())
}

#[test]
fn example() {
    let input = r##"sesenwnenenewseeswwswswwnenewsewsw
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
    assert_eq!(solve(input.as_bytes()).unwrap(), 10);
}
