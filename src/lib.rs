use std::fs::File;
use std::io;
use std::io::BufReader;

pub fn input_file(day: u8, part: u8) -> io::Result<BufReader<File>> {
    let input_path = format!("input/day_{:0>2}/part_{}.txt", day, part);

    Ok(BufReader::new(File::open(input_path)?))
}
