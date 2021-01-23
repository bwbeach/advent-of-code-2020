
use lazy_static;
use regex;
use std::collections;
use std::iter;
use std::fs;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

fn pos(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}

impl iter::Sum for Pos {
    fn sum<I>(iter: I) -> Self 
        where I: Iterator<Item = Pos>
    {
        let mut result = pos(0, 0);
        for p in iter {
            result = result + p;
        }
        result
    }
}

#[test]
fn test_add_pos() {
    assert_eq!(pos(1, 2) + pos(4, 8), pos(5, 10));
}

fn make_dir_to_pos() -> collections::HashMap<String, Pos> {
    let mut result = collections::HashMap::new();
    result.insert(String::from("w"), pos(-1, 0));
    result.insert(String::from("nw"), pos(-1, 1));
    result.insert(String::from("ne"), pos(0, 1));
    result.insert(String::from("e"), pos(1, 0));
    result.insert(String::from("se"), pos(1, -1));
    result.insert(String::from("sw"), pos(0, -1));
    result.insert(String::from("w"), pos(-1, 0));
    result
}

fn parse_directions(directions: &str) -> Pos {
    lazy_static::lazy_static! {
        static ref PATTERN: regex::Regex = regex::Regex::new(r"^[ns]?[ew]").unwrap();
    }
    let dir_to_pos = make_dir_to_pos();
    let mut remaining = directions;
    let mut result = pos(0, 0);
    while 0 < remaining.len() {
        let caps = PATTERN.captures(remaining).unwrap();
        let dir = &caps[0];
        result = result + *dir_to_pos.get(dir).unwrap();
        remaining = &remaining[dir.len()..];
    }
    result
}

#[test]
fn test_parse_directions() {
    assert_eq!(parse_directions("nwwswee"), pos(0, 0));
}

fn run_part1(file_name: &str) -> usize {
    let text = fs::read_to_string(file_name).unwrap();
    let mut black_tiles: collections::HashSet<Pos> = collections::HashSet::new();
    for line in text.split("\n") {
        if ! line.is_empty() {
            let p = parse_directions(line);
            if black_tiles.contains(&p) {
                black_tiles.remove(&p);
            } else {
                black_tiles.insert(p);
            }
        }
    }
    black_tiles.len()
}

fn main() {
    println!("Part 1 sample: {:?}", run_part1("sample.txt"));
    println!("Part 1: {:?}", run_part1("input.txt"));

}
