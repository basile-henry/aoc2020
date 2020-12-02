use std::fs::File;
use std::io;
use std::io::BufReader;

pub fn input_file(day: u8) -> io::Result<BufReader<File>> {
    let input_path = format!("inputs/day_{:0>2}.txt", day);

    Ok(BufReader::new(File::open(input_path)?))
}
