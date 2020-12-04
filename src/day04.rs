use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::num::ParseIntError;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let passport_builders = parse(input)?;

    let solution = match part {
        1 => part_1(&passport_builders),
        2 => part_2(&passport_builders),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

#[derive(Clone, Debug)]
enum Height {
    Cm(usize),
    In(usize),
    NotSpecified(usize),
}

impl Height {
    fn parse(value: &str) -> Result<Self, ParseIntError> {
        let hgt = &value[0..value.len() - 2];
        let unit = &value[value.len() - 2..value.len()];

        Ok(match unit {
            "cm" => Height::Cm(hgt.parse()?),
            "in" => Height::In(hgt.parse()?),
            _ => Height::NotSpecified(value.parse()?),
        })
    }
}

#[derive(Clone, Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<usize>,
}

impl Passport {
    fn valid(&self) -> bool {
        if !matches!(self.byr, 1920..=2002) {
            return false;
        }

        if !matches!(self.iyr, 2010..=2020) {
            return false;
        }

        if !matches!(self.eyr, 2020..=2030) {
            return false;
        }

        match self.hgt {
            Height::Cm(150..=193) => (),
            Height::In(59..=76) => (),
            _ => return false,
        }

        match self.hcl.as_bytes() {
            [b'#', rest @ ..] if rest.len() == 6 => {
                for &byte in rest {
                    match byte {
                        b'0'..=b'9' | b'a'..=b'f' => (),
                        _ => return false,
                    }
                }
            }
            _ => return false,
        }

        match self.ecl.as_ref() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
            _ => return false,
        }

        if self.pid.len() != 9 {
            return false;
        }

        for &byte in self.pid.as_bytes() {
            match byte {
                b'0'..=b'9' => (),
                _ => return false,
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
struct PassportBuilder {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl PassportBuilder {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn parse(&mut self, key_pair: &str) -> Result<(), ParseIntError> {
        let key = &key_pair[0..3];
        let value = &key_pair[4..key_pair.len()];

        match key {
            "byr" => self.byr = Some(value.parse()?),
            "iyr" => self.iyr = Some(value.parse()?),
            "eyr" => self.eyr = Some(value.parse()?),
            "hgt" => self.hgt = Some(Height::parse(value)?),
            "hcl" => self.hcl = Some(value.to_string()),
            "ecl" => self.ecl = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            "cid" => self.cid = Some(value.parse()?),
            _ => unreachable!(), // TODO Better error reporting?
        }

        Ok(())
    }

    fn build(&self) -> Option<Passport> {
        Some(Passport {
            byr: self.byr?,
            iyr: self.iyr?,
            eyr: self.eyr?,
            hgt: self.hgt.clone()?,
            hcl: self.hcl.clone()?,
            ecl: self.ecl.clone()?,
            pid: self.pid.clone()?,
            cid: self.cid,
        })
    }
}

fn parse(input: impl BufRead) -> io::Result<Vec<PassportBuilder>> {
    let mut pb = PassportBuilder::new();
    let mut passport_builders = Vec::new();

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            passport_builders.push(pb.clone());
            pb = PassportBuilder::new();
        } else {
            for key_pair in line.split_whitespace() {
                pb.parse(key_pair)
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            }
        }
    }

    // Push the last one
    passport_builders.push(pb.clone());

    Ok(passport_builders)
}

fn part_1(passport_builders: &[PassportBuilder]) -> usize {
    passport_builders
        .iter()
        .filter(|&pb| pb.build().is_some())
        .count()
}

fn part_2(passport_builders: &[PassportBuilder]) -> usize {
    passport_builders
        .iter()
        .filter_map(|pb| pb.build())
        .filter(|p| p.valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:1937 iyr:2017 cid:147 hgt:183cm

                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                hcl:#cfa07d byr:1929

                hcl:#ae17e1 iyr:2013
                eyr:2024
                ecl:brn pid:760753108 byr:1931
                hgt:179cm

                hcl:#cfa07d eyr:2025 pid:166559648
                iyr:2011 ecl:brn hgt:59in
                ";
        let passport_builders = parse(Cursor::new(input)).unwrap();

        assert_eq!(part_1(&passport_builders), 2)
    }

    #[test]
    fn test_part_2_invalid() {
        let input = "eyr:1972 cid:100
                hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                iyr:2019
                hcl:#602927 eyr:1967 hgt:170cm
                ecl:grn pid:012533040 byr:1946

                hcl:dab227 iyr:2012
                ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                hgt:59cm ecl:zzz
                eyr:2038 hcl:74454a iyr:2023
                pid:3556412378 byr:2007";

        let passport_builders = parse(Cursor::new(input)).unwrap();

        assert_eq!(part_2(&passport_builders), 0)
    }

    #[test]
    fn test_part_2_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f

                eyr:2029 ecl:blu cid:129 byr:1989
                iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                hcl:#888785
                hgt:164cm byr:2001 iyr:2015 cid:88
                pid:545766238 ecl:hzl
                eyr:2022

                iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passport_builders = parse(Cursor::new(input)).unwrap();

        assert_eq!(part_2(&passport_builders), 4)
    }
}
