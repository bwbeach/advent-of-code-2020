
import itertools

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
