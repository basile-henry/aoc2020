use std::io;
use std::io::prelude::*;
use std::num;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let instrs = parse(input).unwrap();

    let solution = match part {
        1 => part_1(&instrs),
        2 => part_2(&instrs).unwrap(),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

type Instrs = Vec<Instr>;

struct CPU {
    accumulator: isize,
    program_counter: usize,
    program: Vec<(Instr, bool)>,
}

impl CPU {
    fn new(instrs: &Instrs) -> Self {
        let program = instrs.iter().map(|&i| (i, false)).collect();

        Self {
            accumulator: 0,
            program_counter: 0,
            program,
        }
    }

    fn step(&mut self) -> bool {
        let (instr, visited) = self.program.get_mut(self.program_counter).unwrap();

        if *visited {
            return false; // Don't step, about to enter a loop
        }

        match *instr {
            Instr::Acc(delta) => {
                self.accumulator += delta;
                self.program_counter += 1;
            }
            Instr::Jmp(delta) => {
                self.program_counter = (self.program_counter as isize + delta) as usize;
            }
            Instr::Nop(_) => self.program_counter += 1,
        }

        // Mark instruction as visited
        *visited = true;

        true
    }
}

fn parse_instr(input: &[u8]) -> Result<Instr> {
    let sign = match input[4] {
        b'-' => -1,
        b'+' => 1,
        _ => return custom_err("No sign")?,
    };
    let amount: isize = std::str::from_utf8(&input[5..])?.parse()?;

    let n = sign * amount;

    Ok(match &input[0..3] {
        b"acc" => Instr::Acc(n),
        b"jmp" => Instr::Jmp(n),
        b"nop" => Instr::Nop(n),
        _ => custom_err(&format!("Instr not supported: {:?}", &input[0..3]))?,
    })
}

fn parse(input: impl BufRead) -> Result<Instrs> {
    input
        .lines()
        .map(|l| try { parse_instr(&l?.as_bytes())? })
        .collect()
}

fn part_1(instrs: &Instrs) -> isize {
    let mut cpu = CPU::new(instrs);

    while cpu.step() {}

    cpu.accumulator
}

fn part_2(instrs: &Instrs) -> Option<isize> {
    let potential_corruption =
        instrs
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, instr)| match instr {
                Instr::Nop(delta) => Some((i, Instr::Jmp(delta))),
                Instr::Jmp(delta) => Some((i, Instr::Nop(delta))),
                _ => None,
            });

    for (i, corrupt) in potential_corruption {
        let mut instrs = instrs.clone();
        instrs[i] = corrupt;

        let mut cpu = CPU::new(&instrs);

        while cpu.step() {
            if cpu.program_counter == instrs.len() {
                return Some(cpu.accumulator);
            }
        }
    }

    None
}

type Result<T> = std::result::Result<T, Day8Error>;

#[derive(Debug)]
enum Day8Error {
    IoError(io::Error),
    ParseError(num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    Custom(String),
}

fn custom_err<T>(e: &str) -> Result<T> {
    Err(Day8Error::Custom(e.to_string()))
}

impl From<io::Error> for Day8Error {
    fn from(error: io::Error) -> Self {
        Day8Error::IoError(error)
    }
}

impl From<num::ParseIntError> for Day8Error {
    fn from(error: num::ParseIntError) -> Self {
        Day8Error::ParseError(error)
    }
}

impl From<std::str::Utf8Error> for Day8Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Day8Error::Utf8Error(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part_1_example() {
        let instrs = parse(io::Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(part_1(&instrs), 5);
    }

    #[test]
    fn part_2_example() {
        let instrs = parse(io::Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(part_2(&instrs).unwrap(), 8);
    }
}
