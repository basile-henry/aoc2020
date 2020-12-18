use std::io;
use std::io::prelude::*;
use std::str::FromStr;

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

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum LExpr {
    Lit(usize),
    Op(Op),
    Nested(Box<Expr>),
}

#[derive(Debug)]
struct Expr {
    row: Vec<LExpr>,
}

fn matching_parens(s: &str) -> usize {
    let mut count = 0;

    for (i, byte) in s.as_bytes().iter().enumerate() {
        match byte {
            b'(' => count += 1,
            b')' if count == 0 => return i,
            b')' => count -= 1,
            _ => (),
        }
    }

    panic!("No matching parens")
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;

        let mut row = Vec::new();

        while !s.is_empty() {
            match s.strip_prefix('(') {
                Some(parens) => {
                    let i = matching_parens(parens);
                    let (nested, rest) = parens.split_at(i);
                    let nested = nested.parse::<Expr>()?;
                    row.push(LExpr::Nested(Box::new(nested)));
                    s = rest[1..].trim_start();
                }
                None => {
                    let word = match s.split_once(' ') {
                        Some((word, rest)) => {
                            s = rest;
                            word
                        }
                        None => {
                            let word = s;
                            s = "";
                            word
                        }
                    };

                    if matches!(word.as_bytes()[0], b'0'..=b'9') {
                        row.push(LExpr::Lit(word.parse().unwrap()));
                    } else if word == "+" {
                        row.push(LExpr::Op(Op::Add));
                    } else if word == "*" {
                        row.push(LExpr::Op(Op::Mul));
                    } else {
                        panic!("Unexpected word: {}", word);
                    }
                }
            }
        }

        Ok(Expr { row })
    }
}

impl LExpr {
    fn eval(self) -> usize {
        match self {
            LExpr::Lit(x) => x,
            LExpr::Op(_) => unreachable!(),
            LExpr::Nested(e) => e.eval(),
        }
    }

    fn eval_add_precedence(self) -> usize {
        match self {
            LExpr::Lit(x) => x,
            LExpr::Op(_) => unreachable!(),
            LExpr::Nested(e) => e.eval_add_precedence(),
        }
    }
}

impl Expr {
    fn eval(self) -> usize {
        let mut row: Vec<LExpr> = self.row.into_iter().rev().collect();
        let mut res = 0;

        while !row.is_empty() {
            match row.pop().unwrap() {
                LExpr::Op(Op::Add) => {
                    let next = row.pop().unwrap();
                    res += next.eval();
                }
                LExpr::Op(Op::Mul) => {
                    let next = row.pop().unwrap();
                    res *= next.eval();
                }
                other => {
                    res = other.eval();
                }
            }
        }

        res
    }

    fn eval_add_precedence(self) -> usize {
        let mut row = self.row;
        let mut res = 0;
        let mut stack = Vec::new();

        while !row.is_empty() {
            match row.pop().unwrap() {
                LExpr::Op(Op::Add) => {
                    let next = row.pop().unwrap();
                    res += next.eval_add_precedence();
                }
                LExpr::Op(Op::Mul) => {
                    stack.push(res);
                }
                other => {
                    res = other.eval_add_precedence();
                }
            }
        }

        stack.push(res);

        stack.iter().product()
    }
}

fn parse(mut input: impl BufRead) -> Vec<Expr> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    input_str.pop(); // Remove last newline
    input_str
        .lines()
        .map(|s| s.parse::<Expr>().unwrap())
        .collect()
}

fn part_1(input: Vec<Expr>) -> usize {
    input.into_iter().map(|e| e.eval()).sum()
}

fn part_2(input: Vec<Expr>) -> usize {
    input.into_iter().map(|e| e.eval_add_precedence()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!("1 + 2 * 3 + 4 * 5 + 6".parse::<Expr>().unwrap().eval(), 71);
        assert_eq!(
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            51
        );
        assert_eq!("2 * 3 + (4 * 5)".parse::<Expr>().unwrap().eval(), 26);
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            437
        );
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            12240
        );
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            13632
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            "1 + 2 * 3 + 4 * 5 + 6"
                .parse::<Expr>()
                .unwrap()
                .eval_add_precedence(),
            231
        );
        assert_eq!(
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<Expr>()
                .unwrap()
                .eval_add_precedence(),
            51
        );
        assert_eq!(
            "2 * 3 + (4 * 5)"
                .parse::<Expr>()
                .unwrap()
                .eval_add_precedence(),
            46
        );
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .eval_add_precedence(),
            1445
        );
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .eval_add_precedence(),
            669060
        );
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .eval_add_precedence(),
            23340
        );
    }
}
