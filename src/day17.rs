use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    let solution = match part {
        1 => part_1(input),
        2 => part_2(input),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(mut input: impl BufRead) -> Grid {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let raw_grid: Vec<Pos> = input_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.as_bytes()
                .into_iter()
                .enumerate()
                .filter(|(_, cell)| **cell == b'#')
                .map(move |(x, _)| (x as isize, y as isize, 0isize, 0isize))
        })
        .collect();

    let (max_x, max_y, _, _) = *raw_grid.last().unwrap();
    let grid = raw_grid.into_iter().collect();

    Grid {
        grid,
        x_bounds: (0, max_x),
        y_bounds: (0, max_y),
        z_bounds: (0, 0),
        w_bounds: (0, 0),
    }
}

type Pos = (isize, isize, isize, isize); // x, y, z, w

#[derive(Debug)]
struct Grid {
    grid: HashSet<Pos>, // Only the active cells
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
    z_bounds: (isize, isize),
    w_bounds: (isize, isize),
}

impl Grid {
    fn _print(&self) {
        for w in self.w_bounds.0..=self.w_bounds.1 {
            for z in self.z_bounds.0..=self.z_bounds.1 {
                println!("z={}, w={}", z, w);
                for y in self.y_bounds.0..=self.y_bounds.1 {
                    for x in self.x_bounds.0..=self.x_bounds.1 {
                        let active = self.grid.contains(&(x, y, z, w));
                        print!("{}", if active { '#' } else { '.' });
                    }
                    println!();
                }
                println!();
            }
        }
    }

    fn neighbours_count(&self, &(x, y, z, w): &Pos) -> usize {
        let mut count = 0;

        for dw in -1..=1 {
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if (dx, dy, dz, dw) != (0, 0, 0, 0) {
                            let active = self.grid.contains(&(x + dx, y + dy, z + dz, w + dw));
                            if active {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn step(&self, dims_4: bool) -> Grid {
        let mut x_bounds = (0, 0);
        let mut y_bounds = (0, 0);
        let mut z_bounds = (0, 0);
        let mut w_bounds = (0, 0);
        let mut grid = HashSet::new();

        let w_iter = if dims_4 {
            (self.w_bounds.0 - 1)..=(self.w_bounds.1 + 1)
        } else {
            0..=0
        };

        for w in w_iter {
            for z in (self.z_bounds.0 - 1)..=(self.z_bounds.1 + 1) {
                for y in (self.y_bounds.0 - 1)..=(self.y_bounds.1 + 1) {
                    for x in (self.x_bounds.0 - 1)..=(self.x_bounds.1 + 1) {
                        let cell = (x, y, z, w);
                        let active = self.grid.contains(&cell);
                        let count = self.neighbours_count(&cell);

                        let new_active = count == 3 || (active && count == 2);

                        if new_active {
                            grid.insert(cell);
                            x_bounds.0 = x_bounds.0.min(x);
                            x_bounds.1 = x_bounds.1.max(x);
                            y_bounds.0 = y_bounds.0.min(y);
                            y_bounds.1 = y_bounds.1.max(y);
                            z_bounds.0 = z_bounds.0.min(z);
                            z_bounds.1 = z_bounds.1.max(z);
                            w_bounds.0 = w_bounds.0.min(w);
                            w_bounds.1 = w_bounds.1.max(w);
                        }
                    }
                }
            }
        }

        Grid {
            grid,
            x_bounds,
            y_bounds,
            z_bounds,
            w_bounds,
        }
    }

    fn run_steps(&mut self, steps: usize, dims_4: bool) {
        for _ in 0..steps {
            *self = self.step(dims_4);
        }
    }
}

fn part_1(mut grid: Grid) -> usize {
    grid.run_steps(6, false);
    grid.grid.len()
}

fn part_2(mut grid: Grid) -> usize {
    grid.run_steps(6, true);
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
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 112);
    }

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 848);
    }
}
