use std::io;
use std::io::prelude::*;

use std::collections::BTreeSet;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let expenses = parse(input)?;

    let solution = match part {
        1 => part_1(&expenses),
        2 => part_2(&expenses),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

type Answers = BTreeSet<char>;
type Group = Vec<Answers>;

fn parse(input: impl BufRead) -> io::Result<Vec<Group>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();

    for line in input.lines() {
        let line = line?;

        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.chars().collect());
        }
    }

    groups.push(group);

    Ok(groups)
}

fn get_group_answers<F>(groups: &[Group], f: F) -> usize
where
    F: Fn(Answers, Answers) -> Answers + Clone,
{
    groups
        .iter()
        .map(|group| group.iter().cloned().fold_first(f.clone()).unwrap().len())
        .sum()
}

fn part_1(groups: &[Group]) -> usize {
    get_group_answers(groups, |group, answers| {
        group.union(&answers).copied().collect()
    })
}

fn part_2(groups: &[Group]) -> usize {
    get_group_answers(groups, |group, answers| {
        group.intersection(&answers).copied().collect()
    })
}
