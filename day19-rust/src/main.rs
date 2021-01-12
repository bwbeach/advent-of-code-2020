// use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn split_pair<'a>(s: &'a str, pattern: &str) -> (&'a str, &'a str) {
    let mut iter = s.split(pattern).filter(|s| ! s.is_empty());
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    assert!(iter.next() == None);
    (first, second)
}

#[derive(Debug, PartialEq)]
enum Pattern<'a> {
    Text(&'a str),
    Choices(Vec<Pattern<'a>>),
    RuleNumbers(Vec<usize>),
}

fn parse_pattern<'a>(text: &'a str) -> Pattern<'a> {
    // lazy_static! {
    //     static ref TEXT_PATTERN: Regex = Regex::new("\"(.*)\"").unwrap();
    // }
    let text_pattern = Regex::new("\"(.*)\"").unwrap();
    if text_pattern.is_match(text) {
        let captures = text_pattern.captures(text).unwrap();
        Pattern::Text(captures.get(1).unwrap().as_str())
    } else if text.contains("|") {
        Pattern::Choices(
            text.split("|").map(|s| parse_pattern(s.trim())).collect()
        )
    } else {
        let numbers: Vec<usize> = text.split(" ")
            .filter(|s| ! s.is_empty())
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        Pattern::RuleNumbers(numbers)
    }
}

fn parse_rule<'a>(text: &'a str) -> (usize, Pattern<'a>) {

    let (num_str, rhs) = split_pair(text, ":");
    let num = num_str.parse::<usize>().unwrap();
    (num, parse_pattern(rhs))
}

#[derive(Debug)]
struct Input<'a> {
    rules: HashMap<usize, Pattern<'a>>,
    messages: Vec<String>,
}

fn parse_input(text: &str) -> Input {
    // The input is in two sections, separated by double newline
    let sections: Vec<_> = text.split("\n\n").collect();
    assert!(sections.len() == 2);

    // The first section is a set of rules, one per line
    let rules: HashMap<usize, Pattern> =
        sections[0].split("\n")
            .filter(|s| ! s.is_empty())
            .map(parse_rule)
            .collect();


    // The second section is a list of messages to check, one per line
    let messages: Vec<String> =
        sections[1].split("\n")
            .filter(|s| ! s.is_empty())
            .map(String::from)
            .collect();

    Input { rules, messages }
}

fn append_regex_to(input: &Input, pattern: &Pattern, output: &mut String) {
    match pattern {
        Pattern::Text(s) => {
            output.push_str(s);
        },
        Pattern::Choices(choices) => {
            output.push_str("(");
            for (i, choice) in choices.iter().enumerate() {
                if i != 0 {
                    output.push_str("|");
                }
                append_regex_to(input, choice, output);
            }
            output.push_str(")");
        },
        Pattern::RuleNumbers(numbers) => {
            for n in numbers {
                append_regex_to(input, input.rules.get(n).unwrap(), output);
            }
        }
    }
}

fn generate_regex(input: &Input) -> String {
    let mut result = String::new();
    result.push_str("^");
    append_regex_to(input, input.rules.get(&0).unwrap(), &mut result);
    result.push_str("$");
    result
}

fn run_part1(input_path: &str) -> usize {
    let input_string = read_to_string(input_path).unwrap();
    let input = parse_input(&input_string);
    let re = Regex::new(&generate_regex(&input)).unwrap();
    input.messages.iter()
        .filter(|m| re.is_match(m))
        .count()
}

fn main() {
    assert_eq!(parse_rule("121: \"a\""), (121, Pattern::Text("a")));
    assert_eq!(
        parse_rule("124: 121 125 | 48 121"), 
        (
            124,
            Pattern::Choices(
                vec![
                     Pattern::RuleNumbers(vec![121, 125]),
                     Pattern::RuleNumbers(vec![48, 121]),
                ]
             )
        )
    );
    
    println!("Sample: {:?}", run_part1("sample.txt"));
    println!("Part 1: {:?}", run_part1("input.txt")); // 203
}
