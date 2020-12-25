use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    match part {
        1 => println!("{}", part_1(parse(input), 100)),
        2 => println!("{}", part_2(parse(input))),
        _ => unimplemented!(),
    };

    Ok(())
}

fn parse<const N: usize>(mut input: impl BufRead) -> Circle<N> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();
    input_str.pop(); // Remove newline
    Circle::parse(&input_str)
}

#[derive(Debug, Clone)]
struct Circle<const N: usize> {
    nexts: Box<[usize]>,
    current_val: usize,
    min: usize,
    max: usize,
}

impl<const N: usize> Circle<N> {
    fn parse(input: &str) -> Self {
        let nexts = vec![0; N + 1]; // 1 value per index + 1 for 0/null
        let mut nexts = nexts.into_boxed_slice();

        let mut min = usize::MAX;
        let mut max = usize::MIN;

        let mut first_value = 0;
        let mut prev_value = 0;

        for (i, x) in input.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let x = x as usize;

            if i == 0 {
                first_value = x;
            } else {
                nexts[prev_value] = x;
            }

            prev_value = x;
            min = min.min(x);
            max = max.max(x);
        }

        for x in max + 1..=N {
            nexts[prev_value] = x;
            prev_value = x;
            max = N;
        }

        nexts[prev_value] = first_value;

        Self {
            min,
            max,
            nexts,
            current_val: first_value,
        }
    }

    fn step(&mut self) {
        // Pickup 3 nexts
        let mut pickup = Vec::new();

        let a = self.nexts[self.current_val];
        pickup.push(a);
        let b = self.nexts[a];
        pickup.push(b);
        let c = self.nexts[b];
        pickup.push(c);

        // Select destination cup
        let mut dest = self.current_val;

        loop {
            dest -= 1;

            if dest < self.min {
                dest = self.max;
            }

            if !pickup.contains(&dest) {
                break;
            }
        }

        // Place picked up cups
        let after_pickup = self.nexts[c];
        let after_dest = self.nexts[dest];

        self.nexts[dest] = a;
        self.nexts[c] = after_dest;
        self.nexts[self.current_val] = after_pickup;

        // Select new current cup (shift circle by one)
        self.current_val = after_pickup;
    }

    fn run_steps(&mut self, moves: usize) {
        for _ in 0..moves {
            self.step();
        }
    }
}

fn part_1(mut circle: Circle<9>, moves: usize) -> String {
    circle.run_steps(moves);

    let mut out = String::new();

    let mut x = 1;

    loop {
        x = circle.nexts[x];

        if x == 1 {
            return out;
        } else {
            out.push_str(&format!("{}", x));
        }
    }
}

fn part_2(mut circle: Circle<1_000_000>) -> u64 {
    circle.run_steps(10_000_000);

    let a = circle.nexts[1];
    let b = circle.nexts[a];

    (a as u64) * (b as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "389125467\n";

    #[test]
    fn part_1_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(&part_1(input.clone(), 10), "92658374");
        assert_eq!(&part_1(input, 100), "67384529");
    }

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 149245887792);
    }
}
