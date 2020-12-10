"""Day 7 of Advent of Code, 2020"""

from day7.day7 import count_contents, find_all_holders, invert_mapping, parse_line

LINE_1 = "light red bags contain 1 bright white bag, 2 muted yellow bags."
LINE_2 = "bright white bags contain 1 shiny gold bag."
LINE_3 = "faded blue bags contain no other bags."


def test_parse_line():
    assert parse_line(LINE_1) == ("light red", {"bright white": 1, "muted yellow": 2})
    assert parse_line(LINE_2) == ("bright white", {"shiny gold": 1})
    assert parse_line(LINE_3) == ("faded blue", {})


def test_invert_mapping():
    actual = invert_mapping({"a": {"b": 2, "c": 3}, "d": {"c": 1}})
    expected = {
        "b": {"a"},
        "c": {"a", "d"},
    }
    assert actual == expected


SAMPLE = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""


def test_sample():
    data = dict(parse_line(line) for line in SAMPLE.splitlines(keepends=False))
    assert find_all_holders("shiny gold", data) == {
        "bright white",
        "muted yellow",
        "light red",
        "dark orange",
    }


PART_2_SAMPLE = """shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"""


def test_part_2():
    data = dict(parse_line(line) for line in PART_2_SAMPLE.splitlines(keepends=False))
    assert count_contents("shiny gold", data) == 126
