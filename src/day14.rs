use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

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

enum Instr {
    Write { addr: usize, value: usize },
    Mask(Vec<u8>),
}

impl Instr {
    fn parse(input: &str) -> Self {
        let (i, payload) = input.split_once(" = ").unwrap();

        match i {
            "mask" => Instr::Mask(payload.as_bytes().to_vec()),
            mem => {
                let addr = mem
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let value = payload.parse::<usize>().unwrap();
                Instr::Write { addr, value }
            }
        }
    }
}

struct State1 {
    mem: HashMap<usize, usize>,
    and: usize,
    or: usize,
}

impl State1 {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
            and: usize::MAX,
            or: usize::MIN,
        }
    }

    fn decode_mask(&mut self, mask: &[u8]) {
        self.and = 0;
        self.or = 0;

        for byte in mask {
            self.and <<= 1;
            self.or <<= 1;

            match byte {
                b'1' => {
                    // Use the "or" mask to add a 1 to a value
                    self.or |= 1;

                    // Make the "and" mask use its default value
                    self.and |= 1;
                }
                b'0' => {
                    // Use the "and" mask to add a 0 to a value
                    self.and |= 0; // Hopefully gets optimized away

                    // Make the "or" mask use its default value
                    self.or |= 0; // Hopefully gets optimized away
                }
                b'X' => {
                    // Make the "and", and "or" mask use their default value
                    self.and |= 1;
                    self.or |= 0; // Hopefully gets optimized away
                }
                _ => unreachable!(),
            }
        }
    }

    fn step(&mut self, instr: &Instr) {
        match instr {
            Instr::Mask(mask) => self.decode_mask(mask),
            Instr::Write { addr, mut value } => {
                value |= self.or;
                value &= self.and;
                self.mem.insert(*addr, value);
            }
        }
    }

    fn run(&mut self, instrs: &[Instr]) {
        instrs.iter().for_each(|i| self.step(i))
    }
}

fn part_1(instrs: &[Instr]) -> usize {
    let mut state = State1::new();
    state.run(instrs);
    state.mem.values().sum()
}

struct State2 {
    mem: HashMap<usize, usize>,
    masks: Vec<(usize, usize)>,
}

impl State2 {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
            masks: Vec::new(),
        }
    }

    fn decode_masks(&mut self, mask: &[u8]) {
        self.masks.clear();
        self.masks.push((0, 0)); // Get 1 pair of masks in

        for byte in mask {
            self.masks.iter_mut().for_each(|(or, and)| {
                *or <<= 1;
                *and <<= 1;
            });

            match byte {
                b'1' => {
                    // Set this bit to 1
                    self.masks.iter_mut().for_each(|(or, and)| {
                        *or |= 1;
                        *and |= 1;
                    });
                }
                b'0' => {
                    // Leave this bit unchanged
                    self.masks.iter_mut().for_each(|(or, and)| {
                        *or |= 0;
                        *and |= 1;
                    });
                }
                b'X' => {
                    // Set this bit to 0
                    self.masks.iter_mut().for_each(|(or, and)| {
                        *or |= 0;
                        *and |= 0;
                    });

                    // Set this bit to 1
                    let mut new = self.masks.clone();
                    new.iter_mut().for_each(|(or, and)| {
                        *or |= 1;
                        *and |= 1;
                    });

                    self.masks.append(&mut new);
                }
                _ => unreachable!(),
            }
        }
    }

    fn step(&mut self, instr: &Instr) {
        match instr {
            Instr::Mask(mask) => self.decode_masks(mask),
            Instr::Write { addr, value } => {
                let addrs = self.masks.iter().map(|(or, and)| (addr & and) | or);
                for addr in addrs {
                    self.mem.insert(addr, *value);
                }
            }
        }
    }

    fn run(&mut self, instrs: &[Instr]) {
        instrs.iter().for_each(|i| self.step(i))
    }
}
fn part_2(instrs: &[Instr]) -> usize {
    let mut state = State2::new();
    state.run(instrs);
    state.mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        const EXAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

        let instrs = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(&instrs), 165);
    }

    #[test]
    fn part_2_example() {
        const EXAMPLE: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

        let instrs = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(&instrs), 208);
    }
}
