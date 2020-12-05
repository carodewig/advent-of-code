""" day 4: passport processing """

import re


def number_between(min_value, max_value):
    def run_for(value):
        try:
            return min_value <= int(value) <= max_value
        except ValueError:
            return False

    return run_for


def validate_height(value):
    if len(value) < 4:
        return False

    unit = value[-2:]
    if unit == "in":
        return number_between(59, 76)(value[:-2])
    if unit == "cm":
        return number_between(150, 193)(value[:-2])

    return False


def matches_regex(regex):
    def run_for(value):
        return bool(re.match(regex, value))

    return run_for


def validate_ecl(value):
    return value in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


FIELD_VALIDATORS = {
    "byr": number_between(1920, 2002),
    "iyr": number_between(2010, 2020),
    "eyr": number_between(2020, 2030),
    "hgt": validate_height,
    "hcl": matches_regex("^#[0-9a-f]{6}$"),
    "ecl": validate_ecl,
    "pid": matches_regex("^[0-9]{9}$"),
}


def check_field_existence(passport_lst):
    mandatory_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    fields_in_passport = [x.split(":")[0] for x in passport_lst]

    for field in mandatory_fields:
        if field not in fields_in_passport:
            return False

    return True


def validate_password(passport_lst):
    if not check_field_existence(passport_lst):
        return False

    for (field, value) in [x.split(":") for x in passport_lst]:
        if not FIELD_VALIDATORS.get(field, lambda x: True)(value):
            return False

    return True


def parse_passports(passports_str, validation_fn):
    passport_str = ""
    for line in passports_str.split("\n"):
        if not line:
            yield validation_fn(passport_str.split())
            passport_str = ""
        else:
            passport_str += " " + line

    yield validation_fn(passport_str.split())


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

REAL_DATA = ""
with open("data/04.txt") as fh:
    REAL_DATA = fh.read()

assert sum(parse_passports(TEST_DATA, check_field_existence)) == 2
assert sum(parse_passports(REAL_DATA, check_field_existence)) == 222


TEST_DATA_INVALID = """
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"""

TEST_DATA_VALID = """
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"""


assert sum(parse_passports(TEST_DATA_INVALID, validate_password)) == 0
assert sum(parse_passports(TEST_DATA_VALID, validate_password)) == 4

assert sum(parse_passports(REAL_DATA, validate_password)) == 140
