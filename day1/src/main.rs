extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::io::prelude::*;


fn main() {
    let matches = App::new("Advent of Code 2017 - Day 1")
        .arg(Arg::with_name("filename")
             .required(true))
        .arg(Arg::with_name("part")
            .possible_values(&["1", "2"]))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let part = matches.value_of("part").unwrap_or("1");

    let mut file = File::open(filename).
        expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong when reading the file");
    contents = contents.trim().to_string();

    let length = contents.len();
    let skip: usize;

    if part == "1" {
        skip = 1;
    } else {
        skip = length / 2;
    }

    let mut total: u32 = 0;
    let mut index: usize = 0;
    while index < length {
        let next = (index + skip) % length;

        let v1 = contents.chars().nth(index).unwrap();
        let v2 = contents.chars().nth(next).unwrap();

        if v1 == v2 {
            total += v1.to_digit(10).expect("Not a digit!");
        }

        index += 1;
    }
    println!("{}", total);
}
