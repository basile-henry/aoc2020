use std::io;
use std::io::prelude::*;

use std::collections::vec_deque::VecDeque;
use std::collections::{HashMap, HashSet};

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let rules = parse(input).unwrap();

    let solution = match part {
        1 => part_1(&rules),
        2 => part_2(&rules),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

type Bag = (String, String);
type Rules = HashMap<Bag, HashMap<Bag, usize>>;

fn parse(input: impl BufRead) -> io::Result<Rules> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();

        match &words[..] {
            &[adj, colour, "bags", "contain", ref rest @ ..] => {
                let key = (adj.to_string(), colour.to_string());
                let mut value = HashMap::new();

                for chunk in rest.chunks(4) {
                    match chunk {
                        &[count, adj, colour, _] => {
                            value.insert(
                                (adj.to_string(), colour.to_string()),
                                count.parse::<usize>().unwrap(),
                            );
                        }
                        &["no", "other", "bags."] => (),
                        _ => unreachable!(),
                    }
                }

                rules.insert(key, value);
            }
            _ => unreachable!(),
        }
    }

    Ok(rules)
}

fn part_1(rules: &Rules) -> usize {
    let shiny = ("shiny".to_string(), "gold".to_string());

    let mut outers: HashMap<Bag, Vec<Bag>> = HashMap::new();

    for (outer, v) in rules {
        for inner in v.keys().cloned() {
            let entry = outers.entry(inner).or_insert(Vec::new());
            entry.push(outer.clone());
        }
    }

    let mut outermost = HashSet::new();
    let mut to_visit = Vec::new();

    to_visit.append(outers.get_mut(&shiny).unwrap());

    while let Some(bag) = to_visit.pop() {
        let next = outers.get_mut(&bag);

        if outermost.insert(bag) {
            if let Some(next) = next {
                to_visit.append(next);
            }
        }
    }

    outermost.len()
}

fn part_2(rules: &Rules) -> usize {
    let mut contains = HashMap::new();
    let mut to_visit: VecDeque<&Bag> = rules.keys().collect();

    while let Some(bag) = to_visit.pop_back() {
        let inside = rules.get(bag).unwrap();

        if inside.is_empty() {
            contains.insert(bag, 1);
        } else {
            let mut known = true;
            let mut count = 0;

            for (e, c) in inside {
                if let Some(o) = contains.get(e) {
                    count += c * o;
                } else {
                    known = false;
                }
            }

            if known {
                contains.insert(bag, count + 1);
            } else {
                to_visit.push_front(bag); // Need to re-visit
            }
        }
    }

    let shiny = ("shiny".to_string(), "gold".to_string());

    *contains.get(&shiny).unwrap() - 1 // don't count the shiny bag itself
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

    #[test]
    fn part_1_example() {
        let rules = parse(io::Cursor::new(EXAMPLE)).unwrap();
        let solution = part_1(&rules);

        assert_eq!(solution, 4);
    }

    #[test]
    fn part_2_example() {
        let rules = parse(io::Cursor::new(EXAMPLE)).unwrap();
        let solution = part_2(&rules);

        assert_eq!(solution, 32);
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let rules = parse(io::Cursor::new(EXAMPLE)).unwrap();

        b.iter(|| part_2(&rules));
    }
}
