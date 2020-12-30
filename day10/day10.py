"""Day 10 of Advent of Code, 2020"""

import more_itertools

from collections import Counter
from typing import Dict, List


def parse_input(input_text: str) -> List[int]:
    """Given the contents of an input file, return the list of numbers in it"""
    return list(int(word) for word in input_text.split())


def get_distribution(adapters: List[int]) -> Dict[int, int]:
    seq = sorted([0] + sorted(adapters) + [max(adapters) + 3])
    return Counter(b - a for a, b in more_itertools.pairwise(seq))


def number_of_combos(adapters: List[int]) -> int:
    seq = sorted([0] + sorted(adapters) + [max(adapters) + 3])
    combos = [0] * len(seq)
    combos[-1] = 1
    for i in range(len(combos) - 2, -1, -1):
        combos[i] = sum(
            combos[j]
            for j in range(i + 1, min(i + 4, len(seq)))
            if 0 < seq[j] - seq[i] <= 3
        )
    return combos[0]


def main():
    with open("input.txt") as f:
        adapters = parse_input(f.read())
    distribution = get_distribution(adapters)
    print("Part 1:", distribution[1] * distribution[3])
    print("Part 2:", number_of_combos(adapters))


if __name__ == "__main__":
    main()
