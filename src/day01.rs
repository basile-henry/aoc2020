use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use std::collections::BTreeSet;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    match part {
        1 => part_1(input)?,
        2 => part_2(input)?,
        _ => unimplemented!(),
    }

    Ok(())
}

pub fn part_1(input: impl BufRead) -> io::Result<()> {
    let mut expenses = BTreeSet::new();

    for line in input.lines() {
        let expense = line?
            .parse::<i32>()
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        let other = 2020 - expense;

        // Find answer
        if expenses.contains(&other) {
            println!("{} * {} = {}", expense, other, expense * other);

            return Ok(());
        }

        expenses.insert(expense);
    }

    Ok(())
}

pub fn part_2(input: impl BufRead) -> io::Result<()> {
    let mut expenses = BTreeSet::new();

    for line in input.lines() {
        let expense = line?
            .parse::<i32>()
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        for entry in &expenses {
            let other = 2020 - expense - entry;

            // Find answer
            if expenses.contains(&other) {
                println!(
                    "{} * {} * {} = {}",
                    expense,
                    entry,
                    other,
                    expense * entry * other
                );

                return Ok(());
            }
        }

        expenses.insert(expense);
    }

    Ok(())
}
