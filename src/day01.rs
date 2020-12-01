use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use std::collections::BTreeSet;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let expenses = parse(input)?;

    let solution = match part {
        1 => part_1(&expenses),
        2 => part_2(&expenses),
        _ => unimplemented!(),
    };

    println!("{}", solution.expect("No solution"));

    Ok(())
}

fn parse(input: impl BufRead) -> io::Result<BTreeSet<i32>> {
    input
        .lines()
        .map(|l| {
            l.and_then(|s| {
                s.parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
            })
        })
        .collect()
}

fn part_1(expenses: &BTreeSet<i32>) -> Option<i32> {
    for expense in expenses {
        let other = 2020 - expense;

        // Find answer
        if expenses.contains(&other) {
            return Some(expense * other);
        }
    }

    None
}

fn part_2(expenses: &BTreeSet<i32>) -> Option<i32> {
    for expense in expenses {
        for other in expenses {
            let another = 2020 - expense - other;

            // Find answer
            if expenses.contains(&another) {
                return Some(expense * other * another);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let expenses = parse(aoc2020::input_file(1, 1).unwrap()).unwrap();

        b.iter(|| part_1(&expenses));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let expenses = parse(aoc2020::input_file(1, 2).unwrap()).unwrap();

        b.iter(|| part_2(&expenses));
    }
}
