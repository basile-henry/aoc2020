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

fn parse(input: impl BufRead) -> io::Result<Vec<i32>> {
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

fn part_1(expenses: &[i32]) -> Option<i32> {
    let mut expenses_set = BTreeSet::new();

    for expense in expenses {
        let other = 2020 - expense;

        // Find answer
        if expenses_set.contains(&other) {
            return Some(expense * other);
        }

        expenses_set.insert(expense);
    }

    None
}

fn part_2(expenses: &[i32]) -> Option<i32> {
    let mut expenses_set = BTreeSet::new();

    for (i, expense) in expenses.iter().enumerate() {
        for other in expenses[i + 1..].iter() {
            let another = 2020 - expense - other;

            // Find answer
            if expenses_set.contains(&another) {
                return Some(expense * other * another);
            }
        }

        expenses_set.insert(expense);
    }

    None
}

// With IterTools.combinations

fn _solve_combinations(input: &[i32], k_combinations: usize, goal: i32) -> Option<i32> {
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

    use std::fs::File;
    use std::io::BufReader;

    fn input() -> BufReader<File> {
        aoc2020::input_file(1).unwrap()
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let expenses = parse(input()).unwrap();

        b.iter(|| part_1(&expenses));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let expenses = parse(input()).unwrap();

        b.iter(|| part_2(&expenses));
    }

    #[test]
    fn part_1_edge_case() {
        let expenses = vec![1010];

        assert!(part_1(&expenses) == None)
    }

    #[test]
    fn part_1_equiv_comb() {
        let expenses = parse(input()).unwrap();

        assert!(part_1(&expenses) == _solve_combinations(&expenses, 2, 2020));
    }

    #[test]
    fn part_2_equiv_comb() {
        let expenses = parse(input()).unwrap();

        assert!(part_2(&expenses) == _solve_combinations(&expenses, 3, 2020));
    }

    #[bench]
    fn bench_part_1_comb(b: &mut Bencher) {
        let expenses = parse(input()).unwrap();

        b.iter(|| _solve_combinations(&expenses, 2, 2020));
    }

    #[bench]
    fn bench_part_2_comb(b: &mut Bencher) {
        let expenses = parse(input()).unwrap();

        b.iter(|| _solve_combinations(&expenses, 3, 2020));
    }
}
