use std::io::prelude::*;
use std::{collections::VecDeque, io};

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    let solution = match part {
        1 => part_1(input),
        2 => part_2(input),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(mut input: impl BufRead) -> Input {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    input_str.pop(); // Remove final newline

    let mut sections = input_str.split("\n\n");
    let rules = Rules::parse(sections.next().unwrap());

    let mut raw_my_ticket = sections.next().unwrap().split('\n');
    let my_ticket = Ticket::parse(raw_my_ticket.nth(1).unwrap());

    let mut raw_nearby_tickets = sections.next().unwrap().split('\n');
    raw_nearby_tickets.next(); // drop section name
    let nearby_tickets = raw_nearby_tickets.map(Ticket::parse).collect();

    Input {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn part_1(input: Input) -> usize {
    input
        .nearby_tickets
        .iter()
        .map(|t| t.check_rules(&input.rules).into_iter().sum::<usize>())
        .sum()
}

fn part_2(input: Input) -> usize {
    let valid_nearby_tickets: Vec<Ticket> = input
        .nearby_tickets
        .iter()
        .filter(|t| t.check_rules(&input.rules).is_empty())
        .cloned()
        .collect();

    let mut known_indices: HashSet<usize> = HashSet::new();
    let mut known_rules: HashMap<String, usize> = HashMap::new();
    let mut to_solve: VecDeque<(String, HashSet<usize>)> = input
        .rules
        .0
        .iter()
        .map(|(k, v)| (k.clone(), v.potential_indices(&valid_nearby_tickets)))
        .collect();

    while !to_solve.is_empty() {
        let (k, mut v) = to_solve.pop_back().unwrap();

        v = v.difference(&known_indices).copied().collect();

        if v.len() > 1 {
            to_solve.push_front((k, v));
        } else {
            let x = *v.iter().next().unwrap();
            known_rules.insert(k, x);
            known_indices.insert(x);
        }
    }

    known_rules
        .into_iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, i)| input.my_ticket.0[i])
        .product()
}

#[derive(Debug, Clone)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn parse(s: &str) -> Self {
        Self(s.split(',').map(|s| s.parse().unwrap()).collect())
    }

    fn potential_indices(&self, rule: &Rule) -> HashSet<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, field)| rule.0.contains(field) || rule.1.contains(field))
            .map(|(i, _)| i)
            .collect()
    }

    fn check_rules(&self, rules: &Rules) -> Vec<usize> {
        self.0
            .iter()
            .filter(|field| {
                let num_matches = rules
                    .0
                    .values()
                    .filter(|rule| rule.0.contains(field) || rule.1.contains(field))
                    .count();

                num_matches == 0
            })
            .copied()
            .collect()
    }
}

#[derive(Debug)]
struct Rule(RangeInclusive<usize>, RangeInclusive<usize>);

impl Rule {
    fn parse(s: &str) -> Self {
        let (a, b) = s.split_once(" or ").unwrap();

        let parse_range = |s: &str| {
            let (start, end) = s.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            RangeInclusive::new(start, end)
        };

        Self(parse_range(a), parse_range(b))
    }

    fn potential_indices(&self, tickets: &[Ticket]) -> HashSet<usize> {
        tickets
            .iter()
            .map(|ticket| ticket.potential_indices(self))
            .fold_first(|a, b| a.intersection(&b).copied().collect())
            .unwrap()
    }
}

#[derive(Debug)]
struct Rules(HashMap<String, Rule>);

impl Rules {
    fn parse(s: &str) -> Self {
        Self(
            s.lines()
                .map(|s| {
                    let (k, v) = s.split_once(": ").unwrap();
                    (k.to_string(), Rule::parse(v))
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
struct Input {
    rules: Rules,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        const EXAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 71);
    }
}
