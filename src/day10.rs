use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let adapters = parse(input);

    let solution = match part {
        1 => part_1(adapters),
        2 => part_2(adapters),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(mut input: impl BufRead) -> Vec<usize> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    input_str
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn part_1(mut adapters: Vec<usize>) -> usize {
    adapters.sort_unstable();

    let offset = std::iter::once(&0).chain(adapters.iter());

    let (c1, c3) = adapters
        .iter()
        .zip(offset)
        .fold((0, 1), |(c1, c3), (a, b)| match a - b {
            1 => (c1 + 1, c3),
            3 => (c1, c3 + 1),
            _ => (c1, c3),
        });

    c1 * c3
}

fn part_2(mut adapters: Vec<usize>) -> usize {
    adapters.sort_unstable();

    let device = adapters.last().unwrap() + 3;

    let initial_state = vec![(1, 0)];

    adapters
        .iter()
        .chain(std::iter::once(&device))
        .fold(initial_state, |mut conns, &x| {
            let mut count = 0;

            for (conn, adapter) in &conns {
                if x - adapter <= 3 {
                    count += conn;
                }
            }

            conns.insert(0, (count, x));
            conns.truncate(3);
            conns
        })[0]
        .0
}
