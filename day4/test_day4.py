
from day4.day4 import parse_file

TEST_DATA = """
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"""

def test_parse():
    passports = parse_file(TEST_DATA)
    assert 4 == len(passports)
    assert {"ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "ght"} == passports[0].field_names
    assert passports[0].has_all_fields({"ecl", "pid"})
    assert not passports[0].has_all_fields({"ecl", "xxx"})
