use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    match part {
        1 => println!("{}", part_1(input)),
        2 => println!("{}", part_2(input)),
        _ => unimplemented!(),
    };

    Ok(())
}

#[derive(Debug)]
struct Input {
    card_public_key: usize,
    door_public_key: usize,
}

fn parse(mut input: impl BufRead) -> Input {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let (card, door) = input_str.trim_end().split_once('\n').unwrap();

    Input {
        card_public_key: card.parse().unwrap(),
        door_public_key: door.parse().unwrap(),
    }
}

fn find_loop_size(subject: usize, modulus: usize, public_key: usize) -> usize {
    let mut loop_size = 1;
    let mut x = subject;

    loop {
        if x == public_key {
            return loop_size;
        }

        x *= subject;
        x %= modulus;
        loop_size += 1;
    }
}

fn find_key(subject: usize, modulus: usize, mut loop_size: usize) -> usize {
    let mut x = subject;

    loop {
        if loop_size == 1 {
            return x;
        }

        x *= subject;
        x %= modulus;
        loop_size -= 1;
    }
}

fn part_1(input: Input) -> usize {
    let modulus = 20201227;
    let door_loop_size = find_loop_size(7, modulus, input.door_public_key);
    find_key(input.card_public_key, modulus, door_loop_size)
}

fn part_2(input: Input) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "5764801\n17807724\n";

    #[test]
    fn simple() {
        let modulus = 20201227;
        let subject = 7;
        assert_eq!(find_loop_size(subject, modulus, 5764801), 8);
        assert_eq!(find_loop_size(subject, modulus, 17807724), 11);
        assert_eq!(find_key(subject, modulus, 8), 5764801);
        assert_eq!(find_key(subject, modulus, 11), 17807724);
    }

    #[test]
    fn part_1_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 14897079);
    }

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 0);
    }
}
