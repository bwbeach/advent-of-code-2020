"""Day 7 of Advent of Code, 2020"""

import re
from collections import defaultdict
from typing import Dict, Set, Tuple

LINE_PATTERN = re.compile(r"^(.*) bags contain (.*)[.]$")
ITEM_PATTERN = re.compile(r"^(\d+) (.*) bags?")


def parse_item(item: str) -> Tuple[str, int]:
    """
    Returns a pair: bag_color, count
    """
    item_match = ITEM_PATTERN.match(item)
    if not item_match:
        raise Exception("Bad item: " + item)
    return item_match.group(2), int(item_match.group(1))


def parse_line(line: str) -> Tuple[str, Dict[str, int]]:
    """
    Returns a pair: bag_color, map_of_color_to_count.

    Raises an exception if the line is not valid.
    """
    line_match = LINE_PATTERN.match(line)
    if not line_match:
        raise Exception("Bad line: " + line)
    bag_color = line_match.group(1)
    contents_str = line_match.group(2)
    if contents_str == "no other bags":
        contents = {}
    else:
        contents = dict(parse_item(item) for item in contents_str.split(", "))
    return (bag_color, contents)


def parse_input() -> Dict[str, Dict[str, int]]:
    """
    Parses the input file, returning a map from bag color to contents.
    """
    with open("input.txt") as input_file:
        file_text = input_file.read()
    return dict(parse_line(line) for line in file_text.splitlines(keepends=False))


def invert_mapping(color_to_contents: Dict[str, Dict[str, int]]) -> Dict[str, Set[str]]:
    """
    Returns a mapping from bag color to bags that can hold it.
    """
    result = defaultdict(set)
    for holder, contents in color_to_contents.items():
        for held in contents.keys():
            result[held].add(holder)
    return result


def transitive_closure(mapping, start):
    """
    Return all of the things you can get to by any number of applications
    of the mapping, starting at 'start'.
    """
    result = set()
    to_check = [start]
    while to_check:
        this_one = to_check.pop()
        for item in mapping[this_one]:
            result.add(item)
            to_check.append(item)
    return result


def find_all_holders(color, data):
    """Returns all of the colors of bags that could hold a given color."""
    color_to_holders = invert_mapping(data)
    return transitive_closure(color_to_holders, color)


def count_contents(color, data):
    """Returns the number of bags that must be inside a bag of a given color"""
    return sum(
        inside_count * (1 + count_contents(inside_color, data))
        for inside_color, inside_count in data[color].items()
    )


def main():
    """Prints the answers for both parts."""
    data = parse_input()
    print(
        "Number that can hold a shiny gold bag:",
        len(find_all_holders("shiny gold", data)),
    )
    print("Number of bags inside a shiny gold bag:", count_contents("shiny gold", data))


if __name__ == "__main__":
    main()
