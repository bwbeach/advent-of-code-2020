
import re

class PasswordEntry:

    line_pattern = re.compile(r"^(\d+)-(\d+) (.): (.*)")

    def __init__(self, line:str) -> None:
        match = self.line_pattern.match(line)
        assert match
        self.first_count = int(match.group(1))
        self.second_count = int(match.group(2))
        self.letter = match.group(3)
        self.password = match.group(4)

    def is_valid_1(self):
        count = sum(1 for c in self.password if c == self.letter)
        return self.first_count <= count <= self.second_count

    def is_valid_2(self):
        first_pos_is_letter = self.password[self.first_count - 1] == self.letter
        second_pos_is_letter = self.password[self.second_count - 1] == self.letter
        return first_pos_is_letter != second_pos_is_letter


with open("input.txt") as f:
    print(
        "First answer:",
        sum(
            1
            for line in f
            if PasswordEntry(line).is_valid_1()
        )
    )

with open("input.txt") as f:
    print(
        "Second answer:",
        sum(
            1
            for line in f
            if PasswordEntry(line).is_valid_2()
        )
    )


