use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::{io, iter::FromIterator};

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

fn parse<T: FromIterator<i32>>(input: impl BufRead) -> io::Result<T> {
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

// With IterTools.combinations

fn solve_combinations(input: &[i32], k_combinations: usize, goal: i32) -> Option<i32> {
    use itertools::Itertools;

    input
        .iter()
        .copied()
        .combinations(k_combinations)
        .find(|v| v.iter().sum::<i32>() == goal)
        .map(|v| v.iter().product::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let expenses: Vec<i32> = parse(aoc2020::input_file(1, 1).unwrap()).unwrap();

        b.iter(|| {
            let expenses = expenses.iter().copied().collect();
            part_1(&expenses)
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let expenses: Vec<i32> = parse(aoc2020::input_file(1, 2).unwrap()).unwrap();

        b.iter(|| {
            let expenses = expenses.iter().copied().collect();

            part_2(&expenses)
        });
    }

    #[test]
    fn part_1_equiv_comb() {
        let expenses_set = parse(aoc2020::input_file(1, 1).unwrap()).unwrap();
        let expenses_vec: Vec<i32> = parse(aoc2020::input_file(1, 1).unwrap()).unwrap();

        assert!(part_1(&expenses_set) == solve_combinations(&expenses_vec, 2, 2020));
    }

    #[test]
    fn part_2_equiv_comb() {
        let expenses_set = parse(aoc2020::input_file(1, 2).unwrap()).unwrap();
        let expenses_vec: Vec<i32> = parse(aoc2020::input_file(1, 2).unwrap()).unwrap();

        assert!(part_2(&expenses_set) == solve_combinations(&expenses_vec, 3, 2020));
    }

    #[bench]
    fn bench_part_1_comb(b: &mut Bencher) {
        let expenses: Vec<i32> = parse(aoc2020::input_file(1, 1).unwrap()).unwrap();

        b.iter(|| solve_combinations(&expenses, 2, 2020));
    }

    #[bench]
    fn bench_part_2_comb(b: &mut Bencher) {
        let expenses: Vec<i32> = parse(aoc2020::input_file(1, 2).unwrap()).unwrap();

        b.iter(|| solve_combinations(&expenses, 3, 2020));
    }
}
