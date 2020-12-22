use std::io;
use std::io::prelude::*;

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    match part {
        1 => println!("{}", part_1(input)),
        2 => println!("{}", part_2(input)),
        _ => unimplemented!(),
    };

    Ok(())
}

type Input = (VecDeque<usize>, VecDeque<usize>);

fn parse(mut input: impl BufRead) -> Input {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let mut player_1 = VecDeque::new();
    let mut player_2 = VecDeque::new();

    let mut player_1_done = false;

    for line in input_str.lines() {
        if !line.starts_with("Player") {
            if line.is_empty() {
                player_1_done = true;
            } else if player_1_done {
                player_2.push_back(line.parse().unwrap())
            } else {
                player_1.push_back(line.parse().unwrap())
            }
        }
    }

    (player_1, player_2)
}

fn winner_score((player_1, player_2): Input) -> usize {
    let winner = if player_1.is_empty() {
        player_2
    } else {
        player_1
    };

    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

fn part_1((mut player_1, mut player_2): Input) -> usize {
    while !(player_1.is_empty() || player_2.is_empty()) {
        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        if p1 > p2 {
            player_1.push_back(p1);
            player_1.push_back(p2);
        } else {
            player_2.push_back(p2);
            player_2.push_back(p1);
        }
    }

    winner_score((player_1, player_2))
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn game(player_1: &mut VecDeque<usize>, player_2: &mut VecDeque<usize>) -> bool {
    let mut seen_p1 = HashSet::new();
    let mut seen_p2 = HashSet::new();

    while !(player_1.is_empty() || player_2.is_empty()) {
        // Check if a player has already had the same hand this round (relies on no hash collisions)
        if !seen_p1.insert(calculate_hash(player_1)) || !seen_p2.insert(calculate_hash(player_2)) {
            return true; // player 1 wins
        }

        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        let sub_game = p1 <= player_1.len() && p2 <= player_2.len();

        let p1_winner = if sub_game {
            game(
                &mut player_1.iter().take(p1).copied().collect(),
                &mut player_2.iter().take(p2).copied().collect(),
            )
        } else {
            p1 > p2
        };

        if p1_winner {
            player_1.push_back(p1);
            player_1.push_back(p2);
        } else {
            player_2.push_back(p2);
            player_2.push_back(p1);
        }
    }

    player_1.len() > player_2.len()
}

fn part_2((mut player_1, mut player_2): Input) -> usize {
    let _ = game(&mut player_1, &mut player_2);

    winner_score((player_1, player_2))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    #[test]
    fn part_1_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 306);
    }

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 291);
    }
}
