use std::fs::read_to_string;
use std::io;
use structopt::StructOpt;

mod day01;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2020", about = "Basile's Advent of Code 2020")]
struct Opt {
    day: u8,
    part: u8,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let input_path = format!("input/day_{:0>2}/part_{}.txt", opt.day, opt.part);
    let input = read_to_string(input_path)?;

    match opt.day {
        1 => day01::solve(&input, opt.part),
        _ => unimplemented!(),
    }

    Ok(())
}
