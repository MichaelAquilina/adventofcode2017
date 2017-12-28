extern crate clap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};


#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
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


#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub condition: Condition,
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

    let instructions = parse(&contents);

    println!("{}", contents);
}


pub fn parse(contents: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in contents.lines() {
        let (operation, condition) = parse_line(line);

        if operation.is_some() && condition.is_some() {
            result.push(
                Instruction {
                    operation: operation.unwrap(),
                    condition: condition.unwrap(),
                });
        }
    }
    result
}


pub fn parse_line(line: &str) -> (Option<Operation>, Option<Condition>) {
    let tokens: Vec<&str> = line.split(" if ")
        .map(|x| x.trim()).collect();

    if tokens.len() != 2 {
        return (None, None);
    }

    let operation = tokens[0];
    let condition = tokens[1];

    let operation = parse_operation(&operation);
    let condition = parse_condition(&condition);

    (operation, condition)
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


pub fn parse_operation(operation: &str) -> Option<Operation> {
    let tokens: Vec<&str> = operation.split(" ").collect();

    if tokens.len() != 3 {
        return None;
    }

    let register = tokens[0].to_string();
    let value = tokens[2].parse().unwrap();
    match tokens[1] {
        "inc" => Some(Operation::Inc(register, value)),
        "dec" => Some(Operation::Dec(register, value)),
        _ => None,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_correct() {
        let result = parse("
g dec 231 if bfx > -10
k dec -567 if wfk == 0
jq inc 880 if a < 2");

        assert_eq!(result.len(), 3);

        assert_eq!(
            result[0], Instruction {
                operation: Operation::Dec(String::from("g"), 231),
                condition: Condition::Larger(String::from("bfx"), -10)
            });
        assert_eq!(
            result[1], Instruction {
                operation: Operation::Dec(String::from("k"), -567),
                condition: Condition::Equal(String::from("wfk"), 0),
            });
        assert_eq!(
            result[2], Instruction {
                operation: Operation::Inc(String::from("jq"), 880),
                condition: Condition::Smaller(String::from("a"), 2),
            });
    }

    #[test]
    fn parse_line_empty() {
        assert_eq!(parse_line(""), (None, None));
    }

    #[test]
    fn parse_line_correct() {
        let (operation, condition) = parse_line("g dec 231 if bfx > -10");

        assert_eq!(
            operation,
            Some(Operation::Dec(String::from("g"), 231))
            );
        assert_eq!(
            condition,
            Some(Condition::Larger(String::from("bfx"), -10))
            );
    }
}
