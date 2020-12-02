#![feature(test)]

extern crate test;

use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

mod day01;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2020", about = "Basile's Advent of Code 2020")]
struct Opt {
    day: u8,
    part: u8,
    #[structopt(long)]
    stdin: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    let input: Box<dyn BufRead> = if opt.stdin {
        Box::new(stdin.lock())
    } else {
        Box::new(aoc2020::input_file(opt.day)?)
    };

    match opt.day {
        1 => day01::solve(input, opt.part)?,
        _ => unimplemented!(),
    }

    Ok(())
}
