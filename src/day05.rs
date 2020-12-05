use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let seats = parse(input).unwrap();

    let solution = match part {
        1 => part_1(&seats),
        2 => part_2(seats),
        _ => unimplemented!(),
    };

    println!("{}", solution.unwrap());

    Ok(())
}

type Seat = u16;

fn parse_seat(input: &str) -> Option<Seat> {
    let mut seat = 0;
    let mut current = 1 << (7 + 3);

    for &byte in input.as_bytes() {
        current >>= 1;

        match byte {
            b'F' => (),
            b'B' => seat |= current,
            b'L' => (),
            b'R' => seat |= current,
            _ => return None,
        }
    }

    Some(seat)
}

fn parse(input: impl BufRead) -> Option<Vec<Seat>> {
    input
        .lines()
        .map(|l| try { parse_seat(&l.ok()?)? })
        .collect()
}

fn part_1(seats: &[Seat]) -> Option<Seat> {
    seats.iter().copied().max()
}

fn part_2(mut seats: Vec<Seat>) -> Option<Seat> {
    seats.sort();

    let mut current = seats.pop()?;

    while let Some(next) = seats.pop() {
        if next == current - 2 {
            return Some(current - 1);
        }

        current = next;
    }

    None
}
