use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let output = input
        .lines()
        .map(str::split_whitespace)
        .filter(|words| {
            let unique_words = words.clone().collect::<HashSet<_>>();
            words.clone().count() - unique_words.len() == 0
        })
        .count();
    println!("{}", output);
}
