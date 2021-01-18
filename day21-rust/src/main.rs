
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

/// Information extracted from a label
#[derive(Debug, Eq, PartialEq)]
struct Label {
    /// All of the ingredients listed on the label
    ingredients: HashSet<String>,

    /// All of the allergen warnings on the label
    warnings: HashSet<String>,
}

/// Parses one input line, containing information from one label
fn parse_label(text: &str) -> Label {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").expect("label regex");
    }
    let captures = PATTERN.captures(text).unwrap();
    let ingredients = captures[1].split(" ").map(|s| String::from(s)).collect();
    let warnings = captures[2].split(", ").map(|s| String::from(s)).collect();
    Label { ingredients, warnings }
}

/// Parses an input file
fn parse_input(file_name: &str) -> Vec<Label> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
        .map(|line| parse_label(&line.unwrap()))
        .collect()
}

#[test]
fn test_parse_label() {
    let mut ingredients: HashSet<String> = HashSet::new();
    ingredients.insert(String::from("mxmxvkd"));
    ingredients.insert(String::from("kfcds"));
    ingredients.insert(String::from("sqjhc"));
    ingredients.insert(String::from("nhms"));

    let mut warnings: HashSet<String> = HashSet::new();
    warnings.insert(String::from("dairy"));
    warnings.insert(String::from("fish"));

    assert_eq!{
        parse_label("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        Label { ingredients, warnings }
    }
}

/// Finds the next assignment of ingredient to allergen
fn find_unique_ingredient<'a, 'b>(
    remaining_ingredients: &'a HashSet<&'b str>,
    allergen: &str,
    labels: &'b [Label]
) -> Option<&'b str> {

    // The ingredients that could have the allergen.  Until
    // we see a label with the allergen, the candidates are
    // ALL of the remaining ingredients.
    let mut candidates: HashSet<&str> = remaining_ingredients.iter().map(|&s| s).collect();

    for label in labels {
        if label.warnings.contains(allergen) {
            candidates = 
                candidates.iter()
                    .filter(|&&ingr| label.ingredients.contains(ingr))
                    .map(|&s| s)
                    .collect();
        }
    }

    if candidates.len() == 1 {
        Some(candidates.iter().next().unwrap())
    } else {
        None
    }
}

/// Finds the next assignment of ingredient to allergen
fn find_next_assignment<'a, 'b>(
    remaining_ingredients: &'a HashSet<&'b str>,
    remaining_allergens: &'a HashSet<&'b str>,
    labels: &'b [Label]
) -> Option<(&'b str, &'b str)> {

    for &allergen in remaining_allergens.iter() {
        if let Some(ingredient) = find_unique_ingredient(remaining_ingredients, allergen, labels) {
            return Some((ingredient, allergen));
        }
    }
    None
}
/// Returns a mapping from ingredient to allergen
fn assign_allergens<'a>(labels: &'a [Label]) -> HashMap<String, String> {
    let mut result = HashMap::new();

    let mut remaining_ingredients: HashSet<&'a str> = 
        labels.iter()
            .flat_map(|label| label.ingredients.iter())
            .map(|s| &s[..])  // is there a better way to convert &String to &str?
            .collect();

    let mut remaining_allergens: HashSet<&str> = 
        labels.iter()
            .flat_map(|label| label.warnings.iter())
            .map(|s| &s[..])  // is there a better way to convert &String to &str?
            .collect();

    while ! remaining_allergens.is_empty() {
        if let Some((ingredient, allergen)) = find_next_assignment(&remaining_ingredients, &remaining_allergens, labels) {
            result.insert(String::from(ingredient), String::from(allergen));
            remaining_ingredients.remove(ingredient);
            remaining_allergens.remove(allergen);
        } else {
            panic!("No next assignment found");
        }
    }

    result
}

/// Counts the number of occurences of ingredients in the given set.
fn part1(labels: &[Label]) -> usize {
    let assignments = assign_allergens(&labels);
    labels.iter()
        // labels
        .flat_map(|label| label.ingredients.iter())
        // ingredients
        .filter(|&ingr| ! assignments.contains_key(ingr))
        // ingredients that were assigned
        .count()
}

fn part2(labels: &[Label]) -> String {
    let mut ingredients_and_allergens: Vec<_> = assign_allergens(&labels).into_iter().collect();
    ingredients_and_allergens.sort_by( |p1, p2| p1.1.cmp(&p2.1) );
    let answer: String = ingredients_and_allergens.iter().map(|(ingr, _)| ingr.as_str()).intersperse(",").collect();
    answer
}

fn main() {
    let sample = parse_input("sample.txt");
    println!("Sample part 1: {:?}", part1(&sample));
    println!("Sample part 2: {:?}", part2(&sample));

    let input = parse_input("input.txt");
    println!("Part 1: {:?}", part1(&input));  // 1882
    println!("Part 2: {:?}", part2(&input));  // xgtj,ztdctgq,bdnrnx,cdvjp,jdggtft,mdbq,rmd,lgllb
}
