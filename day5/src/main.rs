extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of Code - Day 5")
        .arg(Arg::with_name("filename")
             .required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let mut instructions = parse(&contents);
    let result = execute(&mut instructions);

    println!("{}", result);
}


pub fn execute(instructions: &mut Vec<i32>) -> u32 {
    let mut index = 0;
    let mut counter = 0;
    let size = instructions.len() as i32;

    loop {
        counter += 1;
        let jump = instructions[index];
        let next: i32 = (index as i32) + jump;
        if 0 >= next || next >= size {
            break;
        }

        instructions[index] = jump + 1;
        index = next as usize;
    }
    counter
}


pub fn parse(contents: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for token in contents.lines() {
        let value: i32 = token.parse().unwrap();
        result.push(value);
    }
    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_empty() {
        assert_eq!(parse(""), vec![]);
    }

    #[test]
    fn parse_correct() {
        let result = parse("-10\n200\n0\n-300");
        assert_eq!(result, vec![-10, 200, 0, -300]);
    }
}
