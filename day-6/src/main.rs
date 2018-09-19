use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn reallocation(banks: &mut [u32]) {
    let (index, &count) = banks
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, value)| value)
        .unwrap();
    banks[index] = 0;
    let mut index_iter = (0..banks.len()).cycle().skip(index + 1);
    for _ in 0..count {
        banks[index_iter.next().unwrap()] += 1;
    }
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let mut banks: Vec<_> = str::split_whitespace(&input)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut states = HashSet::new();
    let mut count = 0;
    loop {
        if states.contains(&banks) {
            break;
        }
        states.insert(banks.clone());
        reallocation(&mut banks[..]);
        count += 1;
    }
    println!("{:?}", count);
}
