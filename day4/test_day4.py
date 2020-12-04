
from day4.day4 import parse_file, is_field_valid

PART_1_TEST_DATA = """
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
    passports = parse_file(PART_1_TEST_DATA)
    assert 4 == len(passports)
    assert {"ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"} == passports[0].field_names
    assert passports[0].has_all_fields({"ecl", "pid"})
    assert not passports[0].has_all_fields({"ecl", "xxx"})

VALIDATION_TEST_DATA = """
byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789
"""

def test_validate():
    for line in VALIDATION_TEST_DATA.splitlines(keepends=False):
        if line.strip() != "":
            field, validity, value = line.split()
            assert (validity == "valid:") == is_field_valid(field, value)
