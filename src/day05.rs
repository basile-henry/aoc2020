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

fn parse_seat(input: &[u8]) -> Option<Seat> {
    let mut seat = 0;

    for &byte in input {
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
        .map(|l| try { parse_seat(&l.ok()?.as_bytes())? })
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

fn _part_2_one_pass(seats: &str) -> Seat {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let mut sum = 0;

    for seat in seats.as_bytes().chunks(11) {
        let seat = parse_seat(&seat[0..10]).unwrap();

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
        let mut input = aoc2020::input_file(5).unwrap();
        let mut input_str = String::new();
        input.read_to_string(&mut input_str).unwrap();

        b.iter(|| {
            let input = io::Cursor::new(&input_str);
            let seats = parse(input).unwrap();
            part_2_naive(seats.clone())
        });
    }

    #[test]
    fn part_2_equiv() {
        let seats = parse(aoc2020::input_file(5).unwrap()).unwrap();
        let input = include_str!("../inputs/day_05.txt");

        let res = _part_2_one_pass(&input);
        assert_eq!(part_2_naive(seats).unwrap(), res)
    }

    #[bench]
    fn bench_part_2_one_pass(b: &mut Bencher) {
        let input = include_str!("../inputs/day_05.txt");

        b.iter(|| _part_2_one_pass(&input));
    }
}
