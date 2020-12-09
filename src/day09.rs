use std::io;
use std::io::prelude::*;

use std::cmp::Ordering;
use std::collections::VecDeque;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let cipher = parse(input);
    let preamble_size = 25;

    let solution = match part {
        1 => part_1(&cipher, preamble_size),
        2 => part_2(&cipher, preamble_size),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(input: impl BufRead) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug)]
struct CipherCheck {
    preamble: VecDeque<u64>,
    preamble_size: usize,
    working_set: SortedVec<u64>, // Avoid allocations in the main loop
}

#[derive(Debug)]
struct SortedVec<T>(Vec<T>);

impl<T: Ord> SortedVec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    /// Find the index where x should go / where x already is
    fn find(&self, x: &T) -> usize {
        let mut min = 0;
        let mut max = self.0.len();

        while max - min > 1 {
            let mid = (max - min) / 2 + min;

            match self.0[mid].cmp(x) {
                Ordering::Less => {
                    max = mid;
                }
                Ordering::Equal => return mid,
                Ordering::Greater => {
                    min = mid;
                }
            }
        }

        min
    }

    fn contains(&self, x: &T) -> bool {
        if self.0.len() == 0 {
            return false;
        }
        let idx = self.find(x);
        self.0[idx] == *x
    }

    fn insert(&mut self, x: T) {
        let idx = self.find(&x);
        self.0.insert(idx, x);
    }
}

impl CipherCheck {
    fn new(preamble_size: usize) -> Self {
        Self {
            preamble: VecDeque::with_capacity(preamble_size),
            preamble_size,
            working_set: SortedVec::with_capacity(preamble_size),
        }
    }

    fn is_valid(&mut self, x: u64) -> bool {
        if self.preamble.len() < self.preamble_size {
            return true;
        }

        self.working_set.clear();

        for &y in self.preamble.iter() {
            if y < x {
                if self.working_set.contains(&(x - y)) {
                    return true;
                } else {
                    self.working_set.insert(y);
                }
            }
        }

        false
    }

    fn advance(&mut self, x: u64) -> bool {
        if !self.is_valid(x) {
            return false;
        }

        self.preamble.push_back(x);

        if self.preamble.len() > self.preamble_size {
            let _ = self.preamble.pop_front();
        }

        true
    }
}

fn part_1(cipher: &[u64], preamble_size: usize) -> u64 {
    let mut cipher_check = CipherCheck::new(preamble_size);

    for &x in cipher {
        if !cipher_check.advance(x) {
            return x;
        }
    }

    panic!("An answer should have been found by now!");
}

fn part_2(cipher: &[u64], preamble_size: usize) -> u64 {
    let goal = part_1(cipher, preamble_size);

    let mut window = VecDeque::new();
    let mut sum = 0;

    for &x in cipher {
        sum += x;
        window.push_front(x);

        while sum > goal {
            if let Some(y) = window.pop_back() {
                sum -= y;
            }
        }

        if sum == goal {
            break;
        }
    }

    let &min = window.iter().min().unwrap();
    let &max = window.iter().max().unwrap();

    min + max
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const EXAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part_1_example() {
        let cipher = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(&cipher, 5), 127);
    }

    #[test]
    fn part_2_example() {
        let cipher = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(&cipher, 5), 62);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = include_str!("../inputs/day_09.txt");
        let cipher = parse(io::Cursor::new(input));

        b.iter(|| part_1(&cipher, 25));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = include_str!("../inputs/day_09.txt");
        let cipher = parse(io::Cursor::new(input));

        b.iter(|| part_2(&cipher, 25));
    }
}
