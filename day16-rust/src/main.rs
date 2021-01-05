
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Range;
use std::path::Path;

/// A Ticket has an ordered list of numbers
type Ticket = Vec<u64>;

/// A RangeSet is a set of Ranges that say what numbers are allowed
/// in a field on a ticket.
#[derive(Debug, PartialEq)]
struct RangeSet {
    ranges: HashSet<Range<u64>>,
}

impl RangeSet {
    /// Creates a new, empty RangeSet
    fn new() -> RangeSet {
        RangeSet {
            ranges: HashSet::new(),
        }
    }

    /// Adds one range to a range set
    fn insert(&mut self, range: &Range<u64>) -> &mut RangeSet {
        self.ranges.insert(range.clone());
        self
    }

    /// Adds all of the ranges in another range set to this one
    fn insert_all(&mut self, other: &RangeSet) -> &mut RangeSet {
        for r in &other.ranges {
            self.insert(r);
        }
        self
    }

    /// Does one of our ranges contain the given number?
    fn contains(&self, n: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&n))
    }
}

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
    let mut result: RangeSet = RangeSet::new();
    for range_str in s.split(" or ") {
        result.insert(&parse_range(range_str)?);
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
    let other_tickets = 
        lines  
            .map(|line| parse_number_list(&line).unwrap())
            .collect();

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
        if range_set.contains(*n) {
            return true
        }
    }
    false
}

/// Returns the sum of all numbers (from all tickets except ours)
/// that do not match any of the ranges allowed for any field.
fn ticket_scanning_error_rate(input_file: &InputFile) -> u64 {
    // Make a range set containing all of the ranges from
    // the input file
    let mut all_ranges = RangeSet::new();
    for rs in input_file.field_to_range_set.values() {
        all_ranges.insert_all(rs);
    }

    // Sum all of the values that are not in any range.
    input_file.other_tickets
        .iter()
        .flat_map(|t| t.iter())
        .filter(|n| ! all_ranges.contains(**n))
        .sum()
}

fn ticket_has_scan_error(ticket: &Ticket, input_file: &InputFile) -> bool {
    for n in ticket.iter() {
        if ! in_any_range(n, input_file) {
            return true;
        }
    }
    false
}

fn tickets_without_scan_errors(input_file: &InputFile) -> Vec<Ticket> {
    input_file.other_tickets
        .iter()
        .filter(|t| ! ticket_has_scan_error(t, input_file))
        .map(|t| t.clone())
        .collect()
}

fn range_set_matches_column(range_set: &RangeSet, col_index: usize, tickets: &Vec<Ticket>) -> bool {
    for ticket in tickets {
        if ! range_set.contains(ticket[col_index]) {
            return false
        }
    }
    true
}

fn columns_that_match_range_set(tickets: &Vec<Ticket>, range_set: &RangeSet) -> HashSet<usize> {
    let mut result = HashSet::new();
    let column_count = tickets[0].len();
    for col in 0..column_count {
        if range_set_matches_column(range_set, col, tickets) {
            result.insert(col);
        }
    }
    result
}

fn compute_names_and_possible_columns(input_file: &InputFile) -> Vec<(String, HashSet<usize>)> {
    let tickets_to_check = tickets_without_scan_errors(input_file);
    let mut result = Vec::new();
    for (name, range_set) in input_file.field_to_range_set.iter() {
        result.push((String::from(name), columns_that_match_range_set(&tickets_to_check, range_set)));
    }
    result.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    result
}

fn column_order_helper(remaining_fields: &[(String, HashSet<usize>)], columns_used: &HashSet<usize>) -> Option<Vec<String>> {
    let column_count = remaining_fields.len() + columns_used.len();
    if remaining_fields.is_empty() {
        let mut result = Vec::new();
        for _ in 0..column_count {
            result.push(String::new())
        }
        return Some(result)
    }
    let ((field_name, possible_columns), rest) = remaining_fields.split_first().unwrap();
    for candidate in possible_columns {
        if ! columns_used.contains(candidate) {
            let mut more_columns_used = columns_used.clone();
            more_columns_used.insert(*candidate);
            let option_answer: Option<Vec<String>> = column_order_helper(rest, &more_columns_used);
            if option_answer.is_some() {
                let mut answer = option_answer.unwrap();
                answer[*candidate] = String::from(field_name);
                return Some(answer)
            }       
        }
    }
    println!("no solution found for {:?} {:?}", remaining_fields, columns_used);
    None
}
fn column_order(input_file: &InputFile) -> Vec<String> {
    let names_and_possible_columns = compute_names_and_possible_columns(input_file);
    column_order_helper(&names_and_possible_columns, &HashSet::new()).unwrap()
}

fn main() {
    assert_eq!(parse_range("2-10").unwrap(), 2..11);
    assert_eq!(
        parse_range_set("1-4 or 7-8").unwrap(), 
        *RangeSet::new().insert(&(1..5)).insert(&(7..9))
    );
    println!("Hello, world!");

    let sample_input = parse_input_file("sample.txt").unwrap();
    println!("{:?}", sample_input);
    println!("Sample part 1: {:?}", ticket_scanning_error_rate(&sample_input));

    let real_input = parse_input_file("input.txt").unwrap();
    println!("Part 1: {}", ticket_scanning_error_rate(&real_input));
    let column_order = column_order(&real_input);
    println!("Column order: {:?}", column_order);
    let product: u64 = column_order.iter()
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(index, _)| real_input.my_ticket[index])
        .product();
    println!("Part 2: {:?}", product);
}
