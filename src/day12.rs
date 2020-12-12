use std::convert::TryInto;
use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let instrs = parse(input);

    let solution = match part {
        1 => part_1(&instrs),
        2 => part_2(&instrs),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(mut input: impl BufRead) -> Vec<Instr> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    input_str.lines().map(|l| Instr::parse(l)).collect()
}

#[derive(Debug, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn to_angle(&self) -> usize {
        match self {
            Dir::North => 270,
            Dir::East => 0,
            Dir::South => 90,
            Dir::West => 180,
        }
    }

    fn from_angle(angle: usize) -> Self {
        let angle = angle % 360;

        match angle {
            270 => Dir::North,
            0 => Dir::East,
            90 => Dir::South,
            180 => Dir::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Instr {
    Turn(usize),
    Forward(usize),
    Move(Dir, usize),
}

impl Instr {
    fn parse(str: &str) -> Self {
        let (i, n) = str.split_at(1);
        let n = n.parse::<usize>().unwrap();

        match i.as_bytes()[0] {
            b'N' => Instr::Move(Dir::North, n),
            b'E' => Instr::Move(Dir::East, n),
            b'S' => Instr::Move(Dir::South, n),
            b'W' => Instr::Move(Dir::West, n),
            b'R' => Instr::Turn(n),
            b'L' => Instr::Turn(360 - n),
            b'F' => Instr::Forward(n),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    w_x: isize,
    w_y: isize,
    orientation: Dir,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            w_x: 10,
            w_y: 1,
            orientation: Dir::East,
        }
    }

    fn move_towards(&mut self, dir: &Dir, dist: &usize) {
        let dist = *dist as isize;

        match *dir {
            Dir::North => self.y += dist,
            Dir::East => self.x += dist,
            Dir::South => self.y -= dist,
            Dir::West => self.x -= dist,
        }
    }

    fn step(&mut self, instr: &Instr) {
        match instr {
            Instr::Move(dir, n) => self.move_towards(dir, n),
            Instr::Forward(n) => {
                let orientation = self.orientation.clone();
                self.move_towards(&orientation, n);
            }
            Instr::Turn(n) => self.orientation = Dir::from_angle(n + self.orientation.to_angle()),
        }
    }

    fn move_waypoint(&mut self, dir: &Dir, dist: &usize) {
        let dist = *dist as isize;

        match *dir {
            Dir::North => self.w_y += dist,
            Dir::East => self.w_x += dist,
            Dir::South => self.w_y -= dist,
            Dir::West => self.w_x -= dist,
        }
    }

    fn step_waypoint(&mut self, instr: &Instr) {
        match instr {
            Instr::Move(dir, n) => self.move_waypoint(dir, n),
            Instr::Forward(n) => {
                let n = *n as isize;
                self.x += n * self.w_x;
                self.y += n * self.w_y;
            }
            Instr::Turn(n) => match n {
                0 => {}
                90 => {
                    let w_x = self.w_x;
                    self.w_x = self.w_y;
                    self.w_y = -w_x;
                }
                180 => {
                    self.w_x = -self.w_x;
                    self.w_y = -self.w_y;
                }
                270 => {
                    let w_x = self.w_x;
                    self.w_x = -self.w_y;
                    self.w_y = w_x;
                }
                _ => unreachable!(),
            },
        }
    }

    fn manhattan(&self) -> usize {
        (isize::abs(self.x) + isize::abs(self.y))
            .try_into()
            .unwrap()
    }
}

fn part_1(instrs: &[Instr]) -> usize {
    let mut position = Position::new();

    for instr in instrs {
        position.step(instr);
    }

    position.manhattan()
}

fn part_2(instrs: &[Instr]) -> usize {
    let mut position = Position::new();

    for instr in instrs {
        position.step_waypoint(instr);
    }

    position.manhattan()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn part_1_example() {
        let instrs = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(&instrs), 25);
    }

    #[test]
    fn part_2_example() {
        let instrs = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(&instrs), 286);
    }
}
