
import re

PATTERN = re.compile("^([0-9]+)-([0-9]+) ([A-z]): ([A-z]+)$")


def parse_line(line):
    match = re.fullmatch(PATTERN, line.strip())

    if not match:
        return None

    return match.groups()

def valid_password_part1(min_app, max_app, char, password):
    appearances = 0
    for letter in password:
        if letter == char:
            appearances += 1

    return int(min_app) <= appearances <= int(max_app)

def valid_password_part2(pos1, pos2, char, password):
    char1 = password[int(pos1) - 1]
    char2 = password[int(pos2) - 1]

    if char1 == char2:
        return False

    return char in [char1, char2]


def count_valid_passwords(fn, lines):
    count = 0
    for line in lines:
        if parsed := parse_line(line):
            count += int(fn(*parsed))

    return count


test_set = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]

assert count_valid_passwords(valid_password_part1, test_set) == 2
with open("data/02.txt") as fh:
    assert count_valid_passwords(valid_password_part1, fh.readlines()) == 582

assert count_valid_passwords(valid_password_part2, test_set) == 1
with open("data/02.txt") as fh:
    assert count_valid_passwords(valid_password_part2, fh.readlines()) == 729
