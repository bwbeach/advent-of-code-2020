use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use regex::Regex;

type Memory = HashMap<u64, u64>;

/// Holds the parsed value of a bitmask.
#[derive(Debug)]
#[derive(PartialEq)]
struct Mask {
    // The mask to OR with a number to set the 1 bits.
    ones: u64,

    // The mast to AND with the number to clear the 0 bits.
    zeros: u64,
}

/// Applies a bitmask to a number, following the rules int Part 1
fn apply_mask_part1(n: u64, mask: &str) -> u64 {
    let bytes = mask.as_bytes();
    let mut result = n;
    for i in 0usize..36 {
        let c = bytes[35 - i];
        if c == b'0' {
            result &= !(1 << i);
        } else if c == b'1' {
            result |= 1 << i;
        } else if c != b'X' {
            panic!();
        }
    };
    result
}

// Returns a list of the results of applying the mask starting at
// bit index `index`.  All of the previous bits have already been
// processed and the results are in `n`.
fn apply_mask_part2_helper(n: u64, mask: &str, index: usize) -> Vec<u64> {
    if index == 36 {
        vec!(n)
    } else {
        let c = mask.as_bytes()[35 - index];
        if c == b'0' {
            apply_mask_part2_helper(n, mask, index + 1)
        } else if c == b'1' {
            let n_with_bit = n | (1 << (index as u64));
            apply_mask_part2_helper(n_with_bit, mask, index + 1)
        } else if c == b'X' {
            let mut combined: Vec<u64> = Vec::new();
            for x in apply_mask_part2_helper(n & !(1 << index), mask, index + 1) {
                combined.push(x);
            }
            for x in apply_mask_part2_helper(n | (1 << index), mask, index + 1) {
                combined.push(x);
            }
            combined
        } else {
            panic!();
        }

    }
}

/// Applies a bitmask to a number, following the rules int Part 1
fn apply_mask_part2(n: u64, mask: &str) -> Vec<u64> {
    apply_mask_part2_helper(n, mask, 0)
}

/// One line from the input file
#[derive(Debug)]
#[derive(PartialEq)]
enum InputLine {
    Mask(String),
    Store{addr: u64, value: u64}
}

/// Parses one line from the input file
fn parse_input_line(s: &str) -> InputLine {
    if s.starts_with("mask = ") {
        InputLine::Mask(String::from(&s[7..]))
    } 
    else if s.starts_with("mem") {
        // TODO: use a lazy_static for the regx
        let pattern: Regex = Regex::new(r"mem.([0-9]+). *= *([0-9]+)").unwrap();
        let captures = pattern.captures(s).unwrap();
        InputLine::Store{
            addr: captures.get(1).unwrap().as_str().parse::<u64>().unwrap(), 
            value: captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
        }
    } 
    else {
        panic!(format!("unrecognized input line: {:?}", s))
    }
}

/// Processes all of the lines of the input file, and returns the
/// resulting memory.
fn process_input_part1() -> Memory {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut mask: Option<String> = Option::None;
    let mut memory: Memory = HashMap::new();
    for line in reader.lines() { 
        let line_str = line.unwrap();
        match parse_input_line(&line_str) {
            InputLine::Mask(m) => {
                mask = Option::Some(m);
            },
            InputLine::Store{addr, value} => {
                memory.insert(addr, apply_mask_part1(value, &mask.as_ref().unwrap()));
            }
        }
    }
    memory
}

/// Processes all of the lines of the input file, and returns the
/// resulting memory.
fn process_input_part2(path: &Path) -> u64 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut mask: Option<String> = Option::None;
    let mut memory: Memory = HashMap::new();
    for line in reader.lines() { 
        let line_str = line.unwrap();
        match parse_input_line(&line_str) {
            InputLine::Mask(m) => {
                mask = Option::Some(m);
            },
            InputLine::Store{addr, value} => {
                let addresses = apply_mask_part2(addr, &mask.as_ref().unwrap());
                for addr in addresses {
                    memory.insert(addr, value);
                }
            }
        }
    }
    memory.values().sum()
}

fn main() {
    assert_eq!(
        apply_mask_part1(
            0b000000000000000000000000000000001011, 
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        ),
        0b000000000000000000000000000001001001
    );
    assert_eq!(
        apply_mask_part1(
            0b000000000000000000000000000001100101, 
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        ),
        0b000000000000000000000000000001100101
    );
    assert_eq!(
        apply_mask_part1(
            0b000000000000000000000000000000000000, 
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        ),
        0b000000000000000000000000000001000000
    );
    assert_eq!(
        InputLine::Store{addr:45, value:12345},
        parse_input_line("mem[45] = 12345")
    );
    let memory = process_input_part1();
    let part1_answer: u64 = memory.values().sum();
    assert_eq!(12408060320841, part1_answer);
    println!("Part 1: {:?}", part1_answer);
    assert_eq!(208, process_input_part2(Path::new("part2-example.txt")));
    println!("Part 2: {:?}", process_input_part2(Path::new("input.txt")));
}
