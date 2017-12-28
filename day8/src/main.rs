extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};


#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Inc(String, i32),
    Dec(String, i32),
}


#[derive(Debug, Eq, PartialEq)]
pub enum Condition {
    Equal(String, i32),
    Larger(String, i32),
    LargerOrEqual(String, i32),
    Smaller(String, i32),
    SmallerOrEqual(String, i32),
}


fn main() {
    let matches = App::new("Advent of Code 2017 - Day 8")
        .arg(Arg::with_name("filename")
             .required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    println!("{}", contents);
}


pub fn parse_line(line: &str) -> (Option<Instruction>, Option<Condition>) {
    let tokens: Vec<&str> = line.split(" if ")
        .map(|x| x.trim()).collect();

    if tokens.len() != 2 {
        return (None, None);
    }

    let instruction = tokens[0];
    let condition = tokens[1];

    let instruction = parse_instruction(&instruction);
    let condition = parse_condition(&condition);

    (instruction, condition)
}

pub fn parse_condition(condition: &str) -> Option<Condition> {
    let tokens: Vec<&str> = condition.split(" ").collect();

    if tokens.len() != 3 {
        return None;
    }

    let register = tokens[0].to_string();
    let value: i32 = tokens[2].parse().unwrap();

    match tokens[1] {
        "==" => Some(Condition::Equal(register, value)),
        ">" => Some(Condition::Larger(register, value)),
        ">=" => Some(Condition::LargerOrEqual(register, value)),
        "<" => Some(Condition::Smaller(register, value)),
        "<=" => Some(Condition::SmallerOrEqual(register, value)),
        _ => None,
    }
}


pub fn parse_instruction(instruction: &str) -> Option<Instruction> {
    let tokens: Vec<&str> = instruction.split(" ").collect();

    if tokens.len() != 3 {
        return None;
    }

    let register = tokens[0].to_string();
    let value = tokens[2].parse().unwrap();
    match tokens[1] {
        "inc" => Some(Instruction::Inc(register, value)),
        "dec" => Some(Instruction::Dec(register, value)),
        _ => None,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_empty() {
        assert_eq!(parse_line(""), (None, None));
    }

    #[test]
    fn parse_line_correct() {
        let (instruction, condition) = parse_line("g dec 231 if bfx > -10");

        assert_eq!(
            instruction,
            Some(Instruction::Dec(String::from("g"), 231))
            );
        assert_eq!(
            condition,
            Some(Condition::Larger(String::from("bfx"), -10))
            );
    }
}
