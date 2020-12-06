"""
Tests for Day 6 of Advent of Code, 2020
"""


from day6.day6 import conjunction_for_group, disjunction_for_group, group_lines


def test_group_lines():
    """Test grouping lines from the input file."""
    assert group_lines("") == []
    assert group_lines("a\nb\n\nc\n") == [["a", "b"], ["c"]]


def test_count_for_group():
    """Test answering the first question"""
    assert disjunction_for_group([]) == 0
    assert disjunction_for_group(["aabbcc"]) == 3
    assert disjunction_for_group(["abc", "bcd", "def"]) == 6


SECOND_TEST_INPUT = """
abc

a
b
c

ab
ac

a
a
a
a

b
"""


def test_conjunction():
    """Test answering the second question."""
    groups = group_lines(SECOND_TEST_INPUT)
    group_conjunctions = [conjunction_for_group(g) for g in groups]
    assert group_conjunctions == [3, 0, 1, 1, 1]
