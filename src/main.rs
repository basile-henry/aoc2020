#![feature(test)]
#![feature(try_blocks)]
#![feature(str_split_once)]
#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(min_const_generics)]

extern crate test;

use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

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
        5 => day05::solve(input, opt.part)?,
        6 => day06::solve(input, opt.part)?,
        7 => day07::solve(input, opt.part)?,
        8 => day08::solve(input, opt.part)?,
        9 => day09::solve(input, opt.part)?,
        10 => day10::solve(input, opt.part)?,
        11 => day11::solve(input, opt.part)?,
        12 => day12::solve(input, opt.part)?,
        13 => day13::solve(input, opt.part)?,
        14 => day14::solve(input, opt.part)?,
        15 => day15::solve(input, opt.part)?,
        16 => day16::solve(input, opt.part)?,
        17 => day17::solve(input, opt.part)?,
        18 => day18::solve(input, opt.part)?,
        19 => day19::solve(input, opt.part)?,
        20 => day20::solve(input, opt.part)?,
        21 => day21::solve(input, opt.part)?,
        22 => day22::solve(input, opt.part)?,
        23 => day23::solve(input, opt.part)?,
        24 => day24::solve(input, opt.part)?,
        _ => unimplemented!(),
    }

    Ok(())
}
