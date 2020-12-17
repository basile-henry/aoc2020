use std::io;
use std::io::prelude::*;
use std::ops::Add;

use std::collections::HashSet;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let solution = match part {
        1 => part_1(Grid::parse(input)),
        2 => part_2(Grid::parse(input)),
        3 => part_3(Grid::parse(input)),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos<const N: usize>([isize; N]); // x, y, z, w

impl<const N: usize> Pos<N> {
    fn repeated(x: isize) -> Self {
        Self([x; N])
    }

    fn min(&self, other: &Self) -> Self {
        let mut pos = [0; N];

        for (i, s) in self.0.iter().enumerate() {
            pos[i] = *s.min(&other.0[i]);
        }

        Self(pos)
    }

    fn max(&self, other: &Self) -> Self {
        let mut pos = [0; N];

        for (i, s) in self.0.iter().enumerate() {
            pos[i] = *s.max(&other.0[i]);
        }

        Self(pos)
    }
}

impl<const N: usize> Add for Pos<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut pos = [0; N];

        for (i, pos) in pos.iter_mut().enumerate() {
            *pos = self.0[i] + other.0[i];
        }

        Pos(pos)
    }
}

struct PosRange<const N: usize> {
    start: Pos<N>,
    end: Pos<N>,
    current: Option<Pos<N>>,
}

impl<const N: usize> PosRange<N> {
    fn new(start: &Pos<N>, end: &Pos<N>) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
            current: None,
        }
    }
}

impl<const N: usize> Iterator for PosRange<N> {
    type Item = Pos<N>;

    fn next(&mut self) -> Option<Pos<N>> {
        match &mut self.current {
            None => self.current = Some(self.start.clone()),
            Some(c) if *c == self.end => return None,
            Some(c) => {
                let mut i = 0;

                while c.0[i] >= self.end.0[i] {
                    c.0[i] = self.start.0[i];
                    i += 1;
                }

                c.0[i] += 1;
            }
        }

        self.current.clone()
    }
}

#[derive(Debug)]
struct Grid<const N: usize> {
    grid: HashSet<Pos<N>>, // Only the active cells
    min_bound: Pos<N>,
    max_bound: Pos<N>,
}

impl<const N: usize> Grid<N> {
    fn parse(mut input: impl BufRead) -> Self {
        let mut input_str = String::new();
        input.read_to_string(&mut input_str).unwrap();

        let raw_grid: Vec<Pos<N>> = input_str
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.as_bytes()
                    .iter()
                    .enumerate()
                    .filter(|(_, cell)| **cell == b'#')
                    .map(move |(x, _)| {
                        let mut pos = [0; N];
                        pos[0] = x as isize;
                        pos[1] = y as isize;
                        Pos(pos)
                    })
            })
            .collect();

        let max_bound = raw_grid.last().unwrap().clone();
        let grid = raw_grid.into_iter().collect();

        Grid {
            grid,
            min_bound: Pos::repeated(0),
            max_bound,
        }
    }

    fn neighbours_count(&self, pos: &Pos<N>) -> usize {
        let mut count = 0;
        let zero = Pos::repeated(0);
        let iter = PosRange::new(&Pos::repeated(-1), &Pos::repeated(1));

        for dpos in iter {
            if dpos != zero {
                let active = self.grid.contains(&(pos.clone() + dpos));
                if active {
                    count += 1;
                }
            }
        }

        count
    }

    fn step(&self) -> Self {
        let mut grid = HashSet::new();
        let mut min_bound = Pos::repeated(0);
        let mut max_bound = Pos::repeated(0);

        let pos_min_one = Pos::repeated(-1);
        let pos_plus_one = Pos::repeated(1);
        let pos_range = PosRange::new(
            &(self.min_bound.clone() + pos_min_one),
            &(self.max_bound.clone() + pos_plus_one),
        );

        for pos in pos_range {
            let active = self.grid.contains(&pos);
            let count = self.neighbours_count(&pos);

            let new_active = count == 3 || (active && count == 2);

            if new_active {
                min_bound = min_bound.min(&pos);
                max_bound = max_bound.max(&pos);
                grid.insert(pos);
            }
        }

        Grid {
            grid,
            min_bound,
            max_bound,
        }
    }

    fn run_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            *self = self.step();
        }
    }
}

fn part_1(mut grid: Grid<3>) -> usize {
    grid.run_steps(6);
    grid.grid.len()
}

fn part_2(mut grid: Grid<4>) -> usize {
    grid.run_steps(6);
    grid.grid.len()
}

fn part_3(mut grid: Grid<5>) -> usize {
    grid.run_steps(6);
    grid.grid.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = ".#.
..#
###
";

    #[test]
    fn part_1_example() {
        let input = Grid::parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 112);
    }

    #[test]
    fn part_2_example() {
        let input = Grid::parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 848);
    }

    #[test]
    fn part_3_example() {
        let input = Grid::parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_3(input), 5760);
    }
}
