"""
Day 6 of Advent of Code, 2020
"""

import functools
import itertools
from typing import List


def is_blank(line):
    """
    Returns true iff the line contains only whitespace.
    """
    return line.strip() == ""


def group_lines(input_text: str) -> List[List[str]]:
    """
    Given an input file, breaks it into lines and then groups the lines
    into traveling groups.
    """
    return [
        list(group)
        for blank, group in itertools.groupby(input_text.splitlines(), is_blank)
        if not blank
    ]


def disjunction_for_group(lines: List[str]) -> int:
    """
    Returns the number of questions one group answered "yes" to,
    given the lines of input for that group.
    """
    assert all(line.isalpha() for line in lines)
    return len(set(c for c in "".join(lines)))


def conjunction_for_group(lines: List[str]) -> int:
    """
    Returns the number of questions that everybody in the group
    answered yes to.
    """
    sets = [set(line) for line in lines]
    intersection = lambda a, b: a & b
    return len(functools.reduce(intersection, sets))


def main():
    """Reads the input file, and prints the answers"""
    # Read the input file into a single string.
    with open("input.txt") as input_file:
        input_text = input_file.read()

    # Read the input file, and collect groups of lines
    # together, separated by blank lines.
    line_groups = group_lines(input_text)

    # Add the counts from every group.
    print(
        "Sum of all disjunctions:",
        sum(disjunction_for_group(group) for group in line_groups),
    )
    print(
        "Sum of all conjunctions:",
        sum(conjunction_for_group(group) for group in line_groups),
    )


if __name__ == "__main__":
    main()
