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

/// A reader with lookahead.  Unlike a streaming iterator,
/// this returns a copy of the current value, not a ref to it.
struct Reader<'a> {
    iter: &'a mut dyn Iterator<Item = char>,
    curr: Option<char>,
}

impl<'a> Reader<'a> {
    fn new(iter: &'a mut dyn Iterator<Item = char>) -> Self {
        let curr = iter.next();
        Reader { iter, curr }
    }

    fn current(&self) -> Option<char> {
        self.curr
    }

    fn advance(&mut self) {
        self.curr = self.iter.next()
    }

    fn expect_and_skip(&mut self, c: char) {
        assert!(self.curr.unwrap() == c);
        self.advance();
    }
}

/// Applies an operator to two arguments
fn apply_op(op: char, a: i64, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("unknown op: {:?}", op),
    }
}

struct Evaluator {
    /// Sets of operators, by precedence, with least tightly binding first
    op_levels: Vec<Vec<char>>,
}

impl Evaluator {
    /// Evaluates a "primary", which is either a number or a 
    /// parenthesized expression
    fn eval_primary(&self, chars: &mut Reader) -> i64 {
        let c = chars.current().unwrap();
        chars.advance();
        if c == '(' {
            let result = self.eval_ops(0, chars);
            chars.expect_and_skip(')');
            result
        } else if c.is_digit(10) {
            c.to_digit(10).unwrap() as i64
        } else {
            panic!("bad char starting primary: {:?}", c);
        }
    }

    /// Evaluates an expression, going until reaching the end of the
    /// input, or a closing paren.
    fn eval_ops(&self, level: usize, chars: &mut Reader) -> i64 {
        if level == self.op_levels.len() {
            self.eval_primary(chars)
        } else {
            let mut result = self.eval_ops(level + 1, chars);
            loop {
                match chars.current() {
                    None => break,
                    Some(c) => {
                        if self.op_levels[level].contains(&c) {
                            chars.advance();
                            result = apply_op(c, result, self.eval_ops(level + 1, chars));
                        } else {
                            break
                        }
                    },
                }
            }
            result
        }
    }

    /// Evaluates a string containing a complete expression.
    fn eval_string(&self, expr: &str) -> i64 {
        let mut non_space_chars = expr.chars().filter(|c| *c != ' ');
        let mut reader = Reader::new(&mut non_space_chars);
        self.eval_ops(0, &mut reader)
    }
}

/// Returns an iterator over the lines in a file.
fn lines_in_file(file_name: &str) -> Box<dyn Iterator<Item = String>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    Box::new(reader.lines().map(|r| r.unwrap()))
}

fn eval_input(evaluator: &Evaluator) -> i64 {
    lines_in_file("input.txt")
        .map(|line| evaluator.eval_string(&line))
        .sum()
}

fn main() {
    let part1_eval = Evaluator { op_levels: vec![ vec!['+', '*'] ] };
    assert_eq!(part1_eval.eval_string("5"), 5);
    assert_eq!(part1_eval.eval_string("2 * 3 + (4 * 5)"), 26);
    assert_eq!(part1_eval.eval_string("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);

    let part1: i64 = eval_input(&part1_eval);
    println!("Part 1: {:?}", part1);
    assert_eq!(part1, 6811433855019);

    let part2_eval = Evaluator { op_levels: vec![ vec!['*'], vec!['+'] ] };
    assert_eq!(part2_eval.eval_string("5"), 5);
    assert_eq!(part2_eval.eval_string("2 * 3 + (4 * 5)"), 46);
    assert_eq!(part2_eval.eval_string("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);

    let part1: i64 = eval_input(&part2_eval);
    println!("Part 2: {:?}", part1);
}
