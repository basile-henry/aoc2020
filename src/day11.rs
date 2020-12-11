use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let map = Map::parse(input);

    let solution = match part {
        1 => part_1(map),
        2 => part_2(map),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    SeatOccupied,
    SeatEmpty,
    Floor,
}

#[derive(Debug)]
struct Map(Vec<Vec<Cell>>);

impl Map {
    fn parse(mut input: impl BufRead) -> Self {
        let mut input_str = String::new();
        input.read_to_string(&mut input_str).unwrap();

        Self(
            input_str
                .lines()
                .map(|l| {
                    l.as_bytes()
                        .iter()
                        .map(|b| match b {
                            b'#' => Cell::SeatOccupied,
                            b'L' => Cell::SeatEmpty,
                            b'.' => Cell::Floor,
                            _ => panic!("Unexpected byte '{}'", b),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn neighbour_count(&self, x: usize, y: usize, at_distance: bool) -> u8 {
        let mut count = 0;

        for j in -1..=1 {
            for i in -1..=1 {
                let mut dy = j;
                let mut dx = i;

                if dy == 0 && dx == 0 {
                    continue;
                }

                // Using macro instead of closure to get around borrowing issues
                macro_rules! get_cell {
                    () => {
                        self.0
                            .get((y as isize + dy) as usize)
                            .and_then(|v| v.get((x as isize + dx) as usize))
                    };
                }

                if at_distance {
                    while let Some(Cell::Floor) = get_cell!() {
                        dy += j;
                        dx += i;
                    }
                }

                if let Some(Cell::SeatOccupied) = get_cell!() {
                    count += 1;
                }
            }
        }

        count
    }

    fn step(&self, tolerance: u8, at_distance: bool) -> (Self, bool) {
        let mut change = false;
        let map = Map(self
            .0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &cell)| {
                        let count = self.neighbour_count(x, y, at_distance);

                        match cell {
                            Cell::SeatEmpty if count == 0 => {
                                change = true;
                                Cell::SeatOccupied
                            }
                            Cell::SeatOccupied if count >= tolerance => {
                                change = true;
                                Cell::SeatEmpty
                            }
                            _ => cell,
                        }
                    })
                    .collect()
            })
            .collect());
        (map, change)
    }

    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|c| **c == Cell::SeatOccupied).count())
            .sum()
    }
}

fn part_1(mut map: Map) -> usize {
    while let (new_map, true) = map.step(4, false) {
        map = new_map;
    }

    map.count_occupied()
}

fn part_2(mut map: Map) -> usize {
    while let (new_map, true) = map.step(5, true) {
        map = new_map;
    }

    map.count_occupied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn part_1_example() {
        let map = Map::parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(map), 37);
    }

    #[test]
    fn part_2_example() {
        let map = Map::parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(map), 26);
    }
}
