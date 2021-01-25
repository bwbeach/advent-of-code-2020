
use lazy_static;
use regex;
use std::collections;
use std::iter;
use std::fs;
use std::ops;

use conway_life;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

fn pos(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        pos(self.x + other.x, self.y + other.y)
    }
}

impl iter::Sum for Pos {
    fn sum<I>(iter: I) -> Self 
        where I: Iterator<Item = Pos>
    {
        iter.fold(pos(0, 0), |a, b| a + b)
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

/// Structure backing `repeated_regex`
struct RepeatedRegex<'a> {
    /// The pattern to keep matching in the string
    pattern: &'a regex::Regex,
    /// The remaining string to match.  After the first
    /// match, this is the part of the string after the 
    /// previous match.
    remaining: &'a str,
}

/// Iterator backing `repeated_regex`.
/// 
/// The `next()` function finds the next match of the pattern
/// in the remaining string.
impl<'a> Iterator for RepeatedRegex<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.remaining.len() == 0 {
            None
        } else {
            let cap = self.pattern.captures(self.remaining).unwrap();
            let whole_match = cap.get(0).unwrap();
            self.remaining = & self.remaining[whole_match.end()..];
            Some(& whole_match.as_str())
        }
    }
}

/// Returns an iteraror of the strings that match the given pattern
/// in a haystack.  If the pattern starts with `^`, the result is guaranteed
/// to cover every character of the haystack.
/// 
/// No proper error checking (yet).  Panics if the pattern is not found in the haystack.
fn repeated_regex<'a>(pattern: &'a regex::Regex, haystack: &'a str) -> RepeatedRegex<'a> {
    RepeatedRegex { pattern: pattern, remaining: haystack, }
}

fn parse_directions(directions: &str) -> Pos {
    lazy_static::lazy_static! {
        static ref PATTERN: regex::Regex = regex::Regex::new(r"^[ns]?[ew]").unwrap();
    }
    let dir_to_pos = make_dir_to_pos();
    repeated_regex(&PATTERN, directions)
        .map(|dir| *dir_to_pos.get(dir).unwrap())
        .sum()
}

#[test]
fn test_parse_directions() {
    assert_eq!(parse_directions("nwwswee"), pos(0, 0));
}

fn tiles_from_part1(file_name: &str) -> collections::HashSet<Pos> {
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
    black_tiles
}

fn run_part1(file_name: &str) -> usize {
    tiles_from_part1(file_name).len()
}

fn is_alive_part2(was_alive: bool, neighbor_count: usize) -> bool {
    return neighbor_count == 2 || (was_alive && neighbor_count == 1)
}

fn run_part2(file_name: &str) -> usize {
    let neighbors: Vec<_> = make_dir_to_pos().iter().map(|(_, p)| *p).collect();
    let mut tiles = tiles_from_part1(file_name);
    for _ in 1..=100 {
        tiles = conway_life::conway_step(&tiles, neighbors.as_slice(), is_alive_part2);
    }
    tiles.len()
}

fn main() {
    println!("Part 1 sample: {:?}", run_part1("input/day24-sample.txt"));
    println!("Part 2 sample: {:?}", run_part2("input/day24-sample.txt"));
    println!("Part 1: {:?}", run_part1("input/day24-input.txt"));
    println!("Part 2: {:?}", run_part2("input/day24-input.txt"));
}
