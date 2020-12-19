use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Lit(u8), // character
    Series(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_series(s: &str) -> Result<Vec<usize>, ParseIntError> {
            Ok(s.split(' ')
                .map(|n| n.parse())
                .collect::<Result<Vec<usize>, ParseIntError>>()?)
        }

        Ok(if s.starts_with('"') {
            Rule::Lit(s.trim_matches('"').as_bytes()[0])
        } else {
            match s.split_once(" | ") {
                Some((a, b)) => Rule::Or(parse_series(a)?, parse_series(b)?),
                None => Rule::Series(parse_series(s)?),
            }
        })
    }
}

fn match_message<'msg>(idx: &usize, rules: &Rules, msg: &'msg [u8]) -> Vec<&'msg [u8]> {
    let rule = rules.get(idx).unwrap();

    fn match_series<'msg>(rule_idxs: &[usize], rules: &Rules, msg: &'msg [u8]) -> Vec<&'msg [u8]> {
        let mut candidates = vec![msg];
        for rule_idx in rule_idxs {
            let mut new_candidates = Vec::new();

            while let Some(msg) = candidates.pop() {
                let mut next = match_message(rule_idx, rules, msg);
                new_candidates.append(&mut next);
            }

            candidates = new_candidates;
        }

        candidates
    }

    match rule {
        Rule::Lit(b) => {
            if Some(b) == msg.get(0) {
                vec![&msg[1..]]
            } else {
                vec![]
            }
        }
        Rule::Series(rule_idxs) => match_series(rule_idxs, rules, msg),
        Rule::Or(a, b) => {
            let mut matches = match_series(a, rules, msg);
            let mut other_match = match_series(b, rules, msg);
            matches.append(&mut other_match);

            matches
        }
    }
}

type Rules = HashMap<usize, Rule>;
type Input = (Rules, Vec<Vec<u8>>);

fn parse(mut input: impl BufRead) -> Input {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    let mut parsing_rules = true;

    for line in input_str.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let (idx, rule) = line.split_once(": ").unwrap();
            rules.insert(idx.parse().unwrap(), rule.parse().unwrap());
        } else {
            messages.push(line.as_bytes().to_vec());
        }
    }

    (rules, messages)
}

fn part_1((rules, msgs): Input) -> usize {
    msgs.iter()
        .filter(|msg| match_message(&0, &rules, msg).pop() == Some(&[]))
        .count()
}

fn part_2((mut rules, msgs): Input) -> usize {
    rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    msgs.iter()
        .filter(|msg| match_message(&0, &rules, msg).pop() == Some(&[]))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        const EXAMPLE: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";

        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn part_2_example() {
        const EXAMPLE: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
}
";

        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input.clone()), 3);
        assert_eq!(part_2(input), 12);
    }
}
