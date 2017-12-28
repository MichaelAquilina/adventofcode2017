extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};


#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    Inc(i32),
    Dec(i32),
}


#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    register: String,
    operator: Operator,
}


#[derive(Debug, Eq, PartialEq)]
pub enum Comparator {
    Equal(i32),
    Larger(i32),
    LargerOrEqual(i32),
    Smaller(i32),
    SmallerOrEqual(i32),
}


#[derive(Debug, Eq, PartialEq)]
pub struct Condition {
    register: String,
    comparator: Comparator,
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

    let comparator = match tokens[1] {
        "==" => Some(Comparator::Equal(value)),
        ">" => Some(Comparator::Larger(value)),
        ">=" => Some(Comparator::LargerOrEqual(value)),
        "<" => Some(Comparator::Smaller(value)),
        "<=" => Some(Comparator::SmallerOrEqual(value)),
        _ => None,
    };

    if comparator.is_none() {
        return None;
    }
    let comparator = comparator.unwrap();

    Some(Condition { register, comparator })
}


pub fn parse_instruction(instruction: &str) -> Option<Instruction> {
    let tokens: Vec<&str> = instruction.split(" ").collect();

    if tokens.len() != 3 {
        return None;
    }

    let register = tokens[0].to_string();
    let operator = match tokens[1] {
        "inc" => Some(Operator::Inc(tokens[2].parse().unwrap())),
        "dec" => Some(Operator::Dec(tokens[2].parse().unwrap())),
        _ => None,
    };

    if operator.is_none() {
        return None;
    }
    let operator = operator.unwrap();

    Some(Instruction { register, operator })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_correct() {
        let (instruction, condition) = parse_line("g dec 231 if bfx > -10");

        let instruction = instruction.unwrap();
        let condition = condition.unwrap();
        assert_eq!(
            instruction,
            Instruction {
                register: String::from("g"),
                operator: Operator::Dec(231)
            });
        assert_eq!(
            condition,
            Condition {
                register: String::from("bfx"),
                comparator: Comparator::Larger(-10)
            });
    }
}
