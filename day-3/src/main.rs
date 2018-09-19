use std::ops::Add;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let directions = vec![
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
    ];
    let mut directions_iter = directions.iter().cycle().peekable();
    let mut pos = Point {x: 0, y: 0};
    let mut allocated = HashSet::new();
    let mut next = directions_iter.next().unwrap();
    for i in 1..361527 {
        println!("{}: {:?}; dir: {:?}", i, pos, directions_iter.peek().unwrap());
        allocated.insert(pos);
        if !allocated.contains(&(pos + **directions_iter.peek().unwrap())) {
            next = directions_iter.next().unwrap();
        }
        pos = pos + *next;
    }

    let output = pos.x.abs() + pos.y.abs();
    println!("{}", output);
}
