use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    match part {
        1 => println!("{}", part_1(input)),
        2 => println!("{}", part_2(input)),
        _ => unimplemented!(),
    };

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

type Input = Vec<Vec<Dir>>;

fn parse(mut input: impl BufRead) -> Input {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let mut all_dirs = Vec::new();
    let mut dirs = Vec::new();

    for line in input_str.lines() {
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                'e' => dirs.push(Dir::East),
                'w' => dirs.push(Dir::West),
                's' => match chars.next() {
                    Some('e') => dirs.push(Dir::SouthEast),
                    Some('w') => dirs.push(Dir::SouthWest),
                    _ => panic!("Failed to parse South_"),
                },
                'n' => match chars.next() {
                    Some('e') => dirs.push(Dir::NorthEast),
                    Some('w') => dirs.push(Dir::NorthWest),
                    _ => panic!("Failed to parse North_"),
                },
                _ => panic!("Cannot parse {}", c),
            }
        }

        let mut new_dirs = Vec::new();
        std::mem::swap(&mut new_dirs, &mut dirs);
        all_dirs.push(new_dirs);
    }

    all_dirs
}

// Using Cube coordinates: https://www.redblobgames.com/grids/hexagons/#coordinates-cube
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct TileIndex {
    x: isize,
    y: isize,
    z: isize,
}

impl TileIndex {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn move_dir(&mut self, dir: Dir) {
        match dir {
            Dir::East => {
                self.x += 1;
                self.y -= 1;
            }
            Dir::West => {
                self.x -= 1;
                self.y += 1;
            }
            Dir::NorthEast => {
                self.x += 1;
                self.z -= 1;
            }
            Dir::SouthWest => {
                self.x -= 1;
                self.z += 1;
            }
            Dir::NorthWest => {
                self.y += 1;
                self.z -= 1;
            }
            Dir::SouthEast => {
                self.y -= 1;
                self.z += 1;
            }
        }
    }
}

struct Grid(HashSet<TileIndex>);

impl Grid {
    fn from_dirs(input: Input) -> Self {
        let mut black_tiles = HashSet::new();

        for instr in input {
            let mut idx = TileIndex::new();

            for dir in instr {
                idx.move_dir(dir);
            }

            if black_tiles.contains(&idx) {
                black_tiles.remove(&idx);
            } else {
                black_tiles.insert(idx);
            }
        }

        Self(black_tiles)
    }

    fn step(&self) -> Self {
        let mut neighbours = HashMap::<TileIndex, (bool, usize)>::new();

        for &idx in self.0.iter() {
            let entry = neighbours.entry(idx).or_insert((false, 0));
            entry.0 = true;

            use Dir::*;
            for &dir in [East, SouthEast, SouthWest, West, NorthWest, NorthEast].iter() {
                let mut idx = idx;
                idx.move_dir(dir);

                let entry = neighbours.entry(idx).or_insert((false, 0));
                entry.1 += 1;
            }
        }

        Self(
            neighbours
                .into_iter()
                .filter(|(_, (black, count))| *count == 2 || *black && *count == 1)
                .map(|(p, _)| p)
                .collect(),
        )
    }

    fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            *self = self.step();
        }
    }
}

fn part_1(input: Input) -> usize {
    let black_tiles = Grid::from_dirs(input).0;
    black_tiles.len()
}

fn part_2(input: Input) -> usize {
    let mut grid = Grid::from_dirs(input);
    grid.run_steps(100);
    grid.0.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
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
wseweeenwnesenwwwswnew";

    #[test]
    fn part_1_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 10);
    }

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 2208);
    }
}
