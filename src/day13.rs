use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    let solution = match part {
        1 => part_1(input),
        2 => part_2(input.1),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(mut input: impl BufRead) -> (u64, Vec<Option<u64>>) {
    let mut input_str = String::new();

    let _ = input.read_line(&mut input_str);
    input_str.pop(); // Remove newline char
    let earliest_time = input_str.parse().unwrap();

    input_str.clear();
    let _ = input.read_line(&mut input_str);
    input_str.pop(); // Remove newline char
    let buses = input_str
        .split(',')
        .map(|n| {
            if n == "x" {
                None
            } else {
                Some(n.parse::<u64>().unwrap())
            }
        })
        .collect();

    (earliest_time, buses)
}

fn part_1((earliest, buses): (u64, Vec<Option<u64>>)) -> u64 {
    let (b, t) = buses
        .into_iter()
        .filter_map(|b| b.map(|b| (b, b - earliest % b)))
        .min_by_key(|(_, t)| *t)
        .unwrap();

    b * t
}

fn part_2(buses: Vec<Option<u64>>) -> u64 {
    buses
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| (i as u64, b)))
        .fold((0, 1), |(mut offset, period), (bus_offset, bus)| {
            while (offset + bus_offset) % bus != 0 {
                offset += period;
            }

            // Period should use LCM, this is a cheap LCM that assumes prime buses
            (offset, period * bus)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19
";

    #[test]
    fn part_1_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 295);
    }

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input.1), 1068781);
    }
}
