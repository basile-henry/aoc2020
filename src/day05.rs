use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let seats = parse(input).unwrap();

    let solution = match part {
        1 => part_1(&seats),
        2 => part_2_naive(seats),
        _ => unimplemented!(),
    };

    println!("{}", solution.unwrap());

    Ok(())
}

type Seat = usize;

fn parse_seat(input: &str) -> Option<Seat> {
    let mut seat = 0;

    for &byte in input.as_bytes() {
        seat <<= 1;

        match byte {
            b'F' => (),
            b'B' => seat |= 1,
            b'L' => (),
            b'R' => seat |= 1,
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

fn part_2_naive(mut seats: Vec<Seat>) -> Option<Seat> {
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

fn _part_2_one_pass(seats: &[Seat]) -> Seat {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let mut sum = 0;

    for &seat in seats {
        min = seat.min(min);
        max = seat.max(max);
        sum += seat;
    }

    let total_sum = max * (max + 1) / 2 - min * (min - 1) / 2;

    total_sum - sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_2_naive(b: &mut Bencher) {
        let seats = parse(aoc2020::input_file(5).unwrap()).unwrap();

        b.iter(|| part_2_naive(seats.clone()));
    }

    #[test]
    fn part_2_equiv() {
        let seats = parse(aoc2020::input_file(5).unwrap()).unwrap();

        let res = _part_2_one_pass(&seats);
        assert_eq!(part_2_naive(seats).unwrap(), res)
    }

    #[bench]
    fn bench_part_2_one_pass(b: &mut Bencher) {
        let seats = parse(aoc2020::input_file(5).unwrap()).unwrap();

        b.iter(|| _part_2_one_pass(&seats));
    }
}
