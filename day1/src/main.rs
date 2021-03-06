extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::io::prelude::*;


fn main() {
    let matches = App::new("Advent of Code 2017 - Day 1")
        .arg(Arg::with_name("filename")
             .required(true))
        .arg(Arg::with_name("part")
            .possible_values(&["1", "2"]))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let part = matches.value_of("part").unwrap_or("1");

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let skip: usize = match part {
        "1" => 1,
        _ => contents.len() / 2,
    };

    let result = solve(&contents, skip);
    println!("{}", result);
}


fn solve(contents: &str, skip: usize) -> u32 {
    let length = contents.len();
    let mut total: u32 = 0;
    let mut index: usize = 0;
    while index < length {
        let next = (index + skip) % length;

        let v1 = contents.chars().nth(index).unwrap();
        let v2 = contents.chars().nth(next).unwrap();

        if v1 == v2 {
            total += v1.to_digit(10).expect("Not a digit!");
        }

        index += 1;
    }
    return total;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve("1122", 1), 3);
        assert_eq!(solve("1111", 1), 4);
        assert_eq!(solve("22", 1), 4);
        assert_eq!(solve("", 1), 0);
        assert_eq!(solve("1234", 1), 0);
        assert_eq!(solve("91212129", 1), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve("", 0), 0);
        assert_eq!(solve("1212", 2), 6);
        assert_eq!(solve("1221", 2), 0);
        assert_eq!(solve("123425", 3), 4);
        assert_eq!(solve("123123", 3), 12);
        assert_eq!(solve("12131415", 4), 4);
    }
}
