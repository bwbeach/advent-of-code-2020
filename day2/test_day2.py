
from day2.day2 import PasswordEntry

def test_line_pattern():
    entry = PasswordEntry("1-3 a: happy")
    assert 1 == entry.first_count
    assert 3 == entry.second_count
    assert "a" == entry.letter
    assert "happy" == entry.password

def test_is_valid_1():
    assert not PasswordEntry("1-3 a: hppy").is_valid_1()
    assert PasswordEntry("1-3 a: happy").is_valid_1()
    assert PasswordEntry("1-3 a: haaappy").is_valid_1()
    assert not PasswordEntry("1-3 a: haaaappy").is_valid_1()

def test_is_valid_2():
    assert PasswordEntry("1-3 a: abcde").is_valid_2()
    assert not PasswordEntry("1-3 b: cdefg").is_valid_2()
    assert not PasswordEntry("2-9 c: ccccccccc").is_valid_2()
