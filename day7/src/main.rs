extern crate clap;
extern crate regex;

use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};
use regex::Regex;


#[derive(Debug)]
pub struct Tree {
    pub children: HashMap<String, Tree>,
    pub weight: u32,
    pub name: String,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.children == other.children &&
        self.weight == other.weight &&
        self.name == other.name
    }
}

impl Tree {
    pub fn new(name: &str, weight: u32) -> Tree {
        Tree { name: name.to_string(), weight, children: HashMap::new() }
    }

    pub fn parse_leaf(contents: &str) -> (&str, u32) {
        let pattern = Regex::new(r"([a-z]+) \((\d+)\)").unwrap();

        let caps = pattern.captures(contents).unwrap();

        let name = caps.get(1).unwrap().as_str();
        let weight = caps.get(2).unwrap().as_str();

        (name, weight.parse().unwrap())
    }

    pub fn parse_children(contents: &str) -> Vec<&str> {
        contents.split(",").map(|x| x.trim()).collect()
    }

    pub fn parse_line(contents: &str) -> (Tree, Vec<&str>) {
        let tokens: Vec<&str> = contents.split("->").collect();

        let (name, weight) = Tree::parse_leaf(tokens[0].trim());
        let children = if tokens.len() == 2 {
            Tree::parse_children(tokens[1].trim())
        } else {
            vec![]
        };
        (Tree::new(name, weight), children)
    }
}


fn main() {
    let matches = App::new("Advent of Code 2017 - Day 7")
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


#[cfg(test)]
mod test {
    use super::*;

#[test]
    fn parse_line_correct() {
        assert_eq!(
            Tree::parse_line("test (200) -> abcd, foo, bar"),
            (Tree::new("test", 200), vec!["abcd", "foo", "bar"])
        )
    }

    #[test]
    fn parse_leaf_correct() {
        assert_eq!(Tree::parse_leaf("test (200)"), ("test", 200));
    }

    #[test]
    fn parse_children_correct() {
        assert_eq!(
            Tree::parse_children(" hello,  world"),
            vec!["hello", "world"]
        );
    }
}
