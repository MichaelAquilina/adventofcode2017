extern crate clap;
extern crate regex;

use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};
use regex::Regex;


#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    pub parent: Option<String>,
    pub weight: u32,
}


impl Tree {
    pub fn new() -> Tree {
        Tree { weight: 0, parent: None }
    }

    pub fn parse_leaf(contents: &str) -> (String, u32) {
        let pattern = Regex::new(r"([a-z]+) \((\d+)\)").unwrap();

        let caps = pattern.captures(contents).unwrap();

        let name = caps.get(1).unwrap().as_str();
        let weight = caps.get(2).unwrap().as_str();

        (name.to_string(), weight.parse().unwrap())
    }

    pub fn parse_children(contents: &str) -> Vec<String> {
        contents.split(",").map(|x| x.trim().to_string()).collect()
    }

    pub fn parse_line(contents: &str) -> (String, u32, Vec<String>) {
        let tokens: Vec<String> = contents.split("->")
            .map(|x| x.to_string())
            .collect();

        let (name, weight) = Tree::parse_leaf(tokens[0].trim());
        let children = if tokens.len() == 2 {
            Tree::parse_children(tokens[1].trim())
        } else {
            vec![]
        };
        (name, weight, children)
    }

    pub fn parse(contents: &str) -> HashMap<String, Tree> {
        let mut trees: HashMap<String, Tree> = HashMap::new();
        for line in contents.lines() {
            let (name, weight, children) = Tree::parse_line(&line);

            if !trees.contains_key(&name) {
                trees.insert(
                    name.clone(), Tree::new());
            }

            {
                let tree = trees.get_mut(&name).unwrap();
                tree.weight = weight;
            }

            for child in children {
                if !trees.contains_key(&child) {
                    trees.insert(
                        child.clone(), Tree::new());
                }
                let subtree = trees.get_mut(&child).unwrap();
                subtree.parent = Some(name.clone());
            }
        }
        trees
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

    let trees = Tree::parse(&contents);
    for (name, tree) in trees {
        if tree.parent == None {
            println!("{}", name);
            return;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_correct() {
        let result = Tree::parse("
test (200) -> foo, bar
foo (100)
bar (300)".trim());

        assert_eq!(
            result["test"],
            Tree { weight: 200, parent: None });
        assert_eq!(
            result["foo"],
            Tree { weight: 100, parent: Some(String::from("test")) });
        assert_eq!(
            result["bar"],
            Tree { weight: 300, parent: Some(String::from("test")) });
    }

    #[test]
    fn parse_line_correct() {
        assert_eq!(
            Tree::parse_line("test (200) -> abcd, foo, bar"),
            (String::from("test"), 200, vec![
                String::from("abcd"),
                String::from("foo"),
                String::from("bar")])
        )
    }

    #[test]
    fn parse_leaf_correct() {
        assert_eq!(
            Tree::parse_leaf("test (200)"),
            (String::from("test"), 200));
    }

    #[test]
    fn parse_children_correct() {
        assert_eq!(
            Tree::parse_children(" hello,  world"),
            vec!["hello", "world"]
        );
    }
}
