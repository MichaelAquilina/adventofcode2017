extern crate clap;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of Code - Day 4")
        .arg(Arg::with_name("filename")
             .required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut result = 0;
    for line in contents.lines() {
        if valid_passphrase(&line) {
            result += 1;
        }
    }
    println!("{}", result);
}


pub fn valid_passphrase(phrase: &str) -> bool {
    let mut token_set = HashSet::new();
    for token in phrase.split_whitespace() {
        if token_set.contains(token) {
            return false;
        } else {
            token_set.insert(token);
        }
    }
    true
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passphrase() {
        assert!(valid_passphrase(""));
        assert!(valid_passphrase("aa bb cc dd"));
        assert!(!valid_passphrase("aa aa bb"));
    }
}
