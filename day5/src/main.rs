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
    let mut index: i32 = 0;
    let mut counter: u32 = 0;
    let size = instructions.len() as i32;

    loop {
        if index < 0 || index >= size {
            break;
        }

        counter += 1;
        let jump = instructions[index as usize];
        instructions[index as usize] = jump + 1;
        index = index + jump;
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
    fn execute_empty() {
        let mut instructions = vec![];
        assert_eq!(execute(&mut instructions), 0);
    }

    #[test]
    fn execute_correct() {
        let mut instructions = vec![0, 3, 0, 1, -3];
        assert_eq!(execute(&mut instructions), 5);
    }

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
