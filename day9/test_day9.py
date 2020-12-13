from day9.day9 import parse_numbers, find_contiguous_sum, find_non_sum

SAMPLE = """35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"""


def test_find_non_sum():
    assert find_non_sum(parse_numbers(SAMPLE), 5) == 127


def test_find_contiguous_sum():
    assert find_contiguous_sum(parse_numbers(SAMPLE), 127) == [15, 25, 47, 40]
