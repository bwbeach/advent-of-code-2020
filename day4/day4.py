
import itertools
import re


def year_range(low, high):
    """Returns a function that tests whether year is in range, inclusive"""
    return lambda year_str: low <= int(year_str) <= high

def matches_pattern(pattern):
    return lambda x: pattern.match(x) is not None

PID_PATTERN = re.compile(r"^\d{9}$")
HCL_PATTERN = re.compile(r"^#[0-9a-f]{6}$")

def valid_height(x):
    match = re.match(r"^(\d+)(in|cm)$", x)
    if not match:
        return False
    value = int(match.group(1))
    if match.group(2) == "in":
        return 59 <= value <= 76
    else:
        return 150 <= value <= 193

VALIDATORS = {
    "byr": year_range(1920, 2002),
    "iyr": year_range(2010, 2020),
    "eyr": year_range(2020, 2030),
    "hgt": valid_height,
    "hcl": matches_pattern(HCL_PATTERN),
    "ecl": lambda x: x in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"},
    "pid": matches_pattern(PID_PATTERN),
}

def is_field_valid(field_name, value):
    """
    Returns True iff the value is valid for the given field
    """
    return VALIDATORS[field_name](value)


class Passport:
    """
    Holds the key/value mapping for one passport.
    """

    def __init__(self, text):
        """
        Parses one passport.  Input should be a block of text
        with key:value pairs separated by spaces or newlines.
        """
        self.data = dict(
            item.split(":")
            for item in text.split()
        )

    def has_all_fields(self, field_names):
        return field_names <= self.field_names

    def all_fields_valid(self, field_names):
        return all(
            field_name in self.field_names and is_field_valid(field_name, self.data[field_name])
            for field_name in field_names
        )

    @property
    def field_names(self):
        return set(self.data.keys())


def is_blank(line:str):
    return line.strip() == ""


def parse_file(text:str):
    """
    Parses a file containing multiple passports, returning a list
    of Passport objects.
    """
    return [
        Passport("".join(group))
        for key, group in itertools.groupby(text.splitlines(keepends=True), is_blank)
        if not key
    ]

# Count the valid passports in the input file, not requiring the "cid" field.
required_fields = {
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    # "cid",
}
with open("input.txt") as f:
    input_text = f.read()
print(
    "Day 4, part 1:",
    sum(
        1
        for passport in parse_file(input_text)
        if passport.has_all_fields(required_fields)
    )
)
print(
    "Day 4, part 2:",
    sum(
        1
        for passport in parse_file(input_text)
        if passport.all_fields_valid(required_fields)
    )
)
