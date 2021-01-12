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
enum Pattern {
    Text(String),
    Choices(Vec<Pattern>),
    RuleNumbers(Vec<usize>),
}

fn parse_pattern<'a>(text: &'a str) -> Pattern {
    // lazy_static! {
    //     static ref TEXT_PATTERN: Regex = Regex::new("\"(.*)\"").unwrap();
    // }
    let text_pattern = Regex::new("\"(.*)\"").unwrap();
    if text_pattern.is_match(text) {
        let captures = text_pattern.captures(text).unwrap();
        let text = captures.get(1).unwrap().as_str();
        Pattern::Text(String::from(text))
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

fn parse_rule(text: &str) -> (usize, Pattern) {

    let (num_str, rhs) = split_pair(text, ":");
    let num = num_str.parse::<usize>().unwrap();
    (num, parse_pattern(rhs))
}

#[derive(Debug)]
struct Input {
    rules: HashMap<usize, Pattern>,
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

type StrPred<'a> = &'a mut dyn FnMut(&str) -> bool;

fn match_rule_numbers(input: &Input, numbers: &[usize], remaining: &mut StrPred, text: &str) -> bool
{
    if numbers.is_empty() {
        remaining(text)
    } else {
        let mut match_rest: StrPred = 
            &mut |subtext| 
                match_rule_numbers(input, &numbers[1..], remaining, subtext); 
        match_part_2(
            input, 
            input.rules.get(&numbers[0]).unwrap(), 
            &mut match_rest,
            text
        )
    }
}

fn match_part_2(input: &Input, pattern: &Pattern, remaining: &mut StrPred, text: &str) -> bool
{
    match pattern {
        Pattern::Text(s) => {
            if text.starts_with(s) {
                remaining(&text[s.len()..])
            } else {
                false
            }
        },
        Pattern::Choices(choices) => {
            choices.iter().any(
                |choice| match_part_2(input, choice, remaining, text)
            )
        },
        Pattern::RuleNumbers(numbers) => {
            match_rule_numbers(input, numbers, remaining, text)
        }
    }
}

fn parse_input_file(file_name: &str) -> Input {
    let text = read_to_string(file_name).unwrap();
    parse_input(&text)
}

fn run_part1(input_path: &str) -> usize {
    let input = parse_input_file(input_path);
    let re = Regex::new(&generate_regex(&input)).unwrap();
    input.messages.iter()
        .filter(|m| re.is_match(m))
        .count()
}

fn run_new_match_on_part1(input_path: &str) -> usize {
    let input = parse_input_file(&input_path);
    let rule0 = &input.rules[&0];
    let mut match_empty: StrPred = &mut |t| t.is_empty();
    input.messages.iter()
        .filter(|m| match_part_2(&input, rule0, &mut match_empty, m))
        .count()
}

fn run_part2(input_path: &str) -> usize {
    let mut input = parse_input_file(&input_path);
    input.rules.insert(
        8,
        Pattern::Choices(
            vec![
                Pattern::RuleNumbers(vec![42]),
                Pattern::RuleNumbers(vec![42, 8]),
            ]
        )
    );
    input.rules.insert(
        11,
        Pattern::Choices(
            vec![
                Pattern::RuleNumbers(vec![42, 31]),
                Pattern::RuleNumbers(vec![42, 11, 31]),
            ]
        )
    );
    let rule0 = &input.rules[&0];
    let mut match_empty: StrPred = &mut |t| t.is_empty();
    input.messages.iter()
        .filter(|m| match_part_2(&input, rule0, &mut match_empty, m))
        .count()
}

fn main() {
    assert_eq!(parse_rule("121: \"a\""), (121, Pattern::Text(String::from("a"))));
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


    let sample_text = read_to_string("sample.txt").unwrap();
    let sample = parse_input(&sample_text);
    let mut match_empty: StrPred = &mut |t| t.is_empty();
    assert_eq!(match_part_2(&sample, &Pattern::Text(String::from("a")), &mut match_empty, "a"), true);
    assert_eq!(match_part_2(&sample, &Pattern::RuleNumbers(vec![4]), &mut match_empty, "a"), true);
    assert_eq!(match_part_2(&sample, &Pattern::RuleNumbers(vec![4, 5]), &mut match_empty, "ab"), true);
    
    println!("Sample: {:?}", run_part1("sample.txt"));
    println!("Part 1:  {:?}", run_part1("input.txt")); // 203
    println!("Sample b: {:?}", run_new_match_on_part1("sample.txt"));
    println!("Part 1b: {:?}", run_new_match_on_part1("input.txt")); // 203

    println!("Sample 2: {:?}", run_part2("sample2.txt")); // 12
    println!("Part 2: {:?}", run_part2("input.txt"));
}
