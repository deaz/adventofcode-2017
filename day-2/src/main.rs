use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let output: u32 = input
        .lines()
        .map(str::split_whitespace)
        .map(|values| {
            values
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|numbers| {
            numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
        })
        .sum();
    println!("{}", output);
}
