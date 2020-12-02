use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use std::ops::BitXor;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let passwords = parse(input)?;

    let solution = match part {
        1 => part_1(&passwords),
        2 => part_2(&passwords),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

type Password = Vec<u8>;

#[derive(Debug)]
struct Rule {
    min: u8,
    max: u8,
    char: u8,
}

fn parse(input: impl BufRead) -> io::Result<Vec<(Rule, Password)>> {
    input
        .lines()
        .map(|line| try {
            let line = line?;
            let (rule, password) = line.split_once(": ").unwrap();
            let (range, char) = rule.split_once(' ').unwrap();
            let (min, max) = range.split_once('-').unwrap();

            let rule = Rule {
                min: min
                    .parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?,
                max: max
                    .parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?,
                char: char.as_bytes()[0],
            };

            (rule, password.bytes().collect())
        })
        .collect()
}

fn char_count(char: u8, password: &Password) -> usize {
    password.iter().filter(|&&c| c == char).count()
}

fn valid_part_1(rule: &Rule, password: &Password) -> bool {
    let count = char_count(rule.char, password) as u8;

    rule.min <= count && count <= rule.max
}

fn count_valid<F>(passwords: &[(Rule, Password)], valid: F) -> usize
where
    F: Fn(&Rule, &Password) -> bool,
{
    passwords
        .iter()
        .filter(|&(rule, password)| valid(&rule, &password))
        .count()
}

fn part_1(passwords: &[(Rule, Password)]) -> usize {
    count_valid(passwords, valid_part_1)
}

fn valid_part_2(rule: &Rule, password: &Password) -> bool {
    (password[rule.min as usize - 1] == rule.char)
        .bitxor(password[rule.max as usize - 1] == rule.char)
}

fn part_2(passwords: &[(Rule, Password)]) -> usize {
    count_valid(passwords, valid_part_2)
}
