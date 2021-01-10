// Day 18 of Advent of Code 2020.
//
// The task is to evaluate infix expressions where the
// operators all have the same precedence, and are left
// associative.
//
// Assumes that all of the numbers in the input are single
// digits.

use std::io::{BufRead, BufReader};
use std::fs::File;


/// Evaluates a "primary", which is either a number or a 
/// parenthesized expression
fn eval_primary(chars: &mut dyn Iterator<Item = char>) -> i64 {
    let c = chars.next().unwrap();
    if c == '(' {
        eval_until_end_or_paren(chars)
    } else if c.is_digit(10) {
        c.to_digit(10).unwrap() as i64
    } else {
        panic!("bad char starting primary: {:?}", c);
    }
}

/// Evaluates an expression, going until reaching the end of the
/// input, or a closing paren.
fn eval_until_end_or_paren(chars: &mut dyn Iterator<Item = char>) -> i64 {
    let mut result = eval_primary(chars);
    loop {
        match chars.next() {
            None => break,
            Some(c) => {
                if c == ')' {
                    break
                } else {
                    let rhs = eval_primary(chars);
                    match c {
                        '+' => result = result + rhs,
                        '-' => result = result - rhs,
                        '*' => result = result * rhs,
                        '/' => result = result / rhs,
                        _ => panic!("unknown op: {:?}", c),
                    }
                }
            }
        }
    }
    result
}

/// Evaluates a string containing a complete expression.
fn eval_string(expr: &str) -> i64 {
    let mut non_space_chars = expr.chars().filter(|c| *c != ' ');
    eval_until_end_or_paren(&mut non_space_chars)
}

/// Returns an iterator over the lines in a file.
fn lines_in_file(file_name: &str) -> Box<dyn Iterator<Item = String>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    Box::new(reader.lines().map(|r| r.unwrap()))
}

fn main() {
    assert_eq!(eval_string("5"), 5);
    assert_eq!(eval_string("2 * 3 + (4 * 5)"), 26);
    assert_eq!(eval_string("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);

    let part1: i64 =
        lines_in_file("input.txt")
            .map(|line| eval_string(&line))
            .sum();
    println!("Part 1: {:?}", part1);
    assert_eq!(part1, 6811433855019);
}
