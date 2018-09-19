use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let mut numbers = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut index = 0i32;
    loop {
        count += 1;
        numbers[index as usize] += 1;
        index = index + numbers[index as usize] - 1;
        if index < 0 || index >= numbers.len() as i32 {
            break;
        }
    }
    println!("{}", count);
}
