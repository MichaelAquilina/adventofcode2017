extern crate clap;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent of Code - Day 4")
        .arg(Arg::with_name("filename")
             .required(true))
        .arg(Arg::with_name("part")
             .possible_values(&["1", "2"]))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let part = matches.value_of("part").unwrap_or("1");

    let check_anagrams = part == "2";

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut result = 0;
    for line in contents.lines() {
        if valid_passphrase(&line, check_anagrams) {
            result += 1;
        }
    }
    println!("{}", result);
}


pub fn valid_passphrase(phrase: &str, check_anagrams: bool) -> bool {
    let mut token_set = HashSet::new();
    for token in phrase.split_whitespace() {
        let key = if !check_anagrams {
            token.to_string()
        } else {
            let mut c: Vec<char> = token.chars().collect();
            c.sort_unstable();
            c.iter().collect()
        };

        if token_set.contains(&key) {
            return false;
        } else {
            token_set.insert(key);
        }
    }
    true
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passphrase() {
        assert!(valid_passphrase("", false));
        assert!(valid_passphrase("aa bb cc dd", false));
        assert!(!valid_passphrase("aa aa bb", false));

        assert!(valid_passphrase("abcde fghij", true));
        assert!(!valid_passphrase("abcde xyz ecdab", true));
    }
}
