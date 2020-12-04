#![feature(test)]
#![feature(try_blocks)]
#![feature(str_split_once)]
#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]

extern crate test;

use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;

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
        2 => day02::solve(input, opt.part)?,
        3 => day03::solve(input, opt.part)?,
        4 => day04::solve(input, opt.part)?,
        _ => unimplemented!(),
    }

    Ok(())
}
