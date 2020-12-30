"""Day 10 of Advent of Code, 2020"""

from day10.day10 import get_distribution, number_of_combos, parse_input

PART1_SAMPLE_A = """16
10
15
5
1
11
7
19
6
12
4
"""

PART1_SAMPLE_B = """28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"""


def test_get_distribution():
    assert get_distribution(parse_input(PART1_SAMPLE_A)) == {1: 7, 3: 5}
    assert get_distribution(parse_input(PART1_SAMPLE_B)) == {1: 22, 3: 10}


def test_combos():
    assert number_of_combos(parse_input(PART1_SAMPLE_A)) == 8
    assert number_of_combos(parse_input(PART1_SAMPLE_B)) == 19208
