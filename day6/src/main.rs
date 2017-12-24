extern crate clap;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of Code - day 6").
        arg(Arg::with_name("filename")
            .required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();

    let mut memory = parse(&contents);
    let result = count_reallocations(&mut memory);

    println!("{}", result);
}


pub fn max_index(collection: &[u32]) -> Option<usize> {
    if collection.len() == 0 {
        return None;
    }

    let mut max = 0;
    let mut max_index = 0;
    for (index, &value) in collection.iter().enumerate() {
        if value > max {
            max_index = index;
            max = value;
        }
    }
    Some(max_index)
}


pub fn count_reallocations(mut memory: &mut Vec<u32>) -> u32 {
    let mut counter = 0;
    let mut seen: HashSet<Vec<u32>> = HashSet::new();

    loop {
        let key = memory.clone();
        if seen.contains(&key) {
            return counter;
        }
        seen.insert(key);

        counter += 1;
        reallocate(&mut memory);
    }
}


pub fn reallocate(memory: &mut [u32]) {
    // first element with the largest size
    let index = max_index(&memory).unwrap();
    let mut blocks = memory[index];
    memory[index] = 0;

    let mut next = index;
    while blocks > 0 {
        next = (next + 1) % memory.len();
        blocks -= 1;
        memory[next] = memory[next] + 1;
    }
}


pub fn parse(contents: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for token in contents.split_whitespace() {
        let value: u32 = token.parse().unwrap();
        result.push(value);
    };
    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reallocate_correct() {
        let mut memory = vec![0, 2, 7, 0];
        reallocate(&mut memory);
        assert_eq!(memory, &[2, 4, 1, 2]);
        reallocate(&mut memory);
        assert_eq!(memory, &[3, 1, 2, 3]);
        reallocate(&mut memory);
        assert_eq!(memory, &[0, 2, 3, 4]);
    }

    #[test]
    fn count_reallocations_correct() {
        let mut collection = vec![0, 2, 7, 0];
        assert_eq!(count_reallocations(&mut collection), 5);
    }

    #[test]
    fn max_index_empty() {
        let collection = vec![];
        assert_eq!(max_index(&collection), None);
    }

    #[test]
    fn max_index_correct() {
        assert_eq!(max_index(&[1, 2, 4]), Some(2));
        assert_eq!(max_index(&[10, 2, 4]), Some(0));
        assert_eq!(max_index(&[10, 20, 3]), Some(1));
    }

    #[test]
    fn parse_correct() {
        assert_eq!(parse("10   3 4\t5"), vec![10, 3, 4, 5]);
    }
}
