use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let forest = parse(input)?;

    let solution = match part {
        1 => part_1(&forest),
        2 => part_2(&forest),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

type Forest = Vec<Vec<bool>>;

fn parse(input: impl BufRead) -> io::Result<Forest> {
    input
        .lines()
        .map(|line| try { line?.bytes().map(|b| b == b'#').collect() })
        .collect()
}

fn trees_in_slope(forest: &Forest, dx: usize, dy: usize) -> usize {
    forest
        .iter()
        .step_by(dy)
        .enumerate()
        .filter(|(i, row)| {
            let x = i * dx % row.len();
            row[x]
        })
        .count()
}

fn part_1(forest: &Forest) -> usize {
    trees_in_slope(forest, 3, 1)
}

fn part_2(forest: &Forest) -> usize {
    trees_in_slope(forest, 1, 1)
        * trees_in_slope(forest, 3, 1)
        * trees_in_slope(forest, 5, 1)
        * trees_in_slope(forest, 7, 1)
        * trees_in_slope(forest, 1, 2)
}
