extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};


fn main() {
    let matches = App::new("Advent of Code - Day 2")
        .arg(Arg::with_name("filename")
             .index(1))
        .arg(Arg::with_name("part")
             .possible_values(&["1", "2"]))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let part = matches.value_of("part").unwrap_or("1");

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let result = match part {
        "1" => verify_p1(&contents),
        "2" => verify_p2(&contents),
        _ => {
            println!("Unknown part specified");
            0
        }
    };
    println!("{}", result);
}

pub fn parse(contents: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for token in contents.split_whitespace() {
        if token != "" {
            result.push(token.parse().unwrap());
        }
    }
    result
}


pub fn verify_p1(contents: &str) -> u32 {
    let mut result: u32 = 0;
    for line in contents.lines() {
        let values = parse(&line);
        let max_value = values.iter().max().unwrap_or(&0);
        let min_value = values.iter().min().unwrap_or(&0);
        result += max_value - min_value;
    }
    result
}


pub fn verify_p2(contents: &str) -> u32 {
    let mut result: u32 = 0;
    for line in contents.lines() {
        let values = parse(&line);
        for x in values.iter() {
            for y in values.iter() {
                if x != y && x % y == 0 {
                    result += x / y;
                }
            }
        }
    }
    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse(""), vec![]);
        assert_eq!(parse("8\t10\t20\t3"), vec![8, 10, 20, 3]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(verify_p1("5 9 2 8"), 7);
        assert_eq!(verify_p1("
5 1 9 5
7 5 3
2 4 6 8"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(verify_p2("5 9 2 8"), 4);
        assert_eq!(verify_p2("
5 9 2 8
9 4 7 3
3 8 6 5"), 9);
    }
}
