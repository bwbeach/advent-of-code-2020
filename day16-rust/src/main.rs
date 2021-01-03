
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Range;
use std::path::Path;

/// A Ticket has an ordered list of numbers
type Ticket = Vec<u64>;

/// A RangeSet is a set of Ranges that say what numbers are allowed
/// in a field on a ticket.
type RangeSet = Vec<Range<u64>>;

fn parse_int(s: &str) -> Option<u64> {
    s.parse::<u64>().ok()
}

/// Turns a string like "1-10", with inclusive start and end, into a Range
fn parse_range(s: &str) -> Option<Range<u64>> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^([0-9]+)-([0-9]+)$").expect("range regex");
    }
    let captures = PATTERN.captures(s)?;
    let start = parse_int(captures.get(1)?.as_str())?;
    let end = parse_int(captures.get(2)?.as_str())?;
    Some(start..(end + 1))
}

/// Turns a string like "1-4 or 7-8" into a Vec of ranges.
fn parse_range_set(s: &str) -> Option<RangeSet> {
    let mut result: RangeSet = Vec::new();
    for range_str in s.split(" or ") {
        result.push(parse_range(range_str)?);
    }
    Some(result)
}

fn parse_field_line(s: &str) -> Option<(String, RangeSet)> {
    let colon_pos = s.find(":")?;
    Some((String::from(&s[..colon_pos]), parse_range_set(&s[colon_pos+2 ..])?))
}

/// Structure holding everything in an input file
#[derive(Debug)]
struct InputFile {
    // mapping from field name on ticket to range
    field_to_range_set: HashMap<String, RangeSet>,

    // the numbers on my ticket
    my_ticket: Ticket,

    // all of the other tickets
    other_tickets: Vec<Ticket>,
}

/// Turns a comma-separated list of numbers into a Ticket
fn parse_number_list(s: &str) -> Option<Ticket> {
    let mut result = Vec::new();
    for number_str in s.split(",") {
        result.push(parse_int(number_str)?);
    }
    Some(result)
}

/// Parses an entire input file
fn parse_input_file(path_str: &str) -> Option<InputFile> {

    // Open the file for reading and make a line iterator
    let path = Path::new(path_str);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(&file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    // read the ranges on each field
    let mut field_to_range_set = HashMap::new();
    loop {
        let line: String = lines.next()?;
        if line.is_empty() {
            break
        }
        let (field, range_set) = parse_field_line(line.as_str())?;
        field_to_range_set.insert(field, range_set);
    }

    // my ticket
    assert_eq!("your ticket:", lines.next()?);
    let my_ticket = parse_number_list(&lines.next()?)?;
    assert_eq!("", lines.next()?);

    // other tickets
    assert_eq!("nearby tickets:", lines.next()?);
    let mut other_tickets = Vec::new();
    for line in lines {
        other_tickets.push(parse_number_list(&line)?);
    }

    Some(
        InputFile{
            field_to_range_set: field_to_range_set,
            my_ticket: my_ticket,
            other_tickets: other_tickets,
        }
    )
}

fn in_any_range(n: &u64, input_file: &InputFile) -> bool {
    for range_set in input_file.field_to_range_set.values() {
        for range in range_set.iter() {
            if range.contains(n) {
                return true
            }
        }
    }
    false
}

fn ticket_scanning_error_rate(input_file: &InputFile) -> u64 {
    let mut result = 0;
    for ticket in input_file.other_tickets.iter() {
        for n in ticket.iter() {
            if ! in_any_range(n, input_file) {
                result += n;
            }
        }
    }
    result
}

fn ranges_match_column(input_file: &InputFile, col_index: usize, tickets: &Vec<Ticket>) -> bool {
    for ticket in tickets {
        if ! in_any_range(&ticket[col_index], input_file) {
            return false;
        }
    }
    true
}

fn column_order(input_file: &InputFile, col_index: usize, names: &[&str]) -> Option<Vec<String>> {
    let mut first_and_rest: Vec<&str> = Vec::new();
    for name in names {
        first_and_rest.push(name);
    }
    for i in 0..first_and_rest.len() {
        // move this candidate into position
        first_and_rest.swap(0, i);

        // is it compatible with the data?
        let first_name = *first_and_rest.get(0).unwrap();
        if ranges_match_column(input_file, col_index, &input_file.other_tickets) {
            if let Some(rest_columns) = column_order(input_file, col_index + 1, &first_and_rest[1..]) {
                let mut answer = Vec::new();
                answer.push(String::from(first_name));
                for later_name in rest_columns {
                    answer.push(String::from(later_name));
                }
                return Some(answer)
            }
        }
    }
    panic!("no answer found")
}

fn main() {
    assert_eq!(parse_range("2-10").unwrap(), 2..11);
    assert_eq!(parse_range_set("1-4 or 7-8").unwrap(), [1..5, 7..9]);
    println!("Hello, world!");

    let sample_input = parse_input_file("sample.txt").unwrap();
    println!("{:?}", sample_input);
    println!("Sample part 1: {:?}", ticket_scanning_error_rate(&sample_input));

    let real_input = parse_input_file("input.txt").unwrap();
    println!("Part 1: {}", ticket_scanning_error_rate(&real_input));
    let column_names: Vec<String> = real_input.field_to_range_set.keys().map(|s| String::from(s)).collect();
    let column_name_refs: Vec<&str> = column_names.iter().map(|s| s.as_str()).collect();
    println!("Part 2 column order: {:?}", column_order(&real_input, 0, &column_name_refs));
}
