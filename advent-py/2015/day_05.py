""" day 5: doesn't he have intern-elves for this? """

import re

VOWELS = re.compile(r"[aeiou]")
DOUBLE_LETTER = re.compile(r"(.)\1")
DISALLOWED_STRINGS = re.compile(r"(ab|cd|pq|xy)")


DOUBLE_PAIR = re.compile(r"(..).*\1")
SANDWICH_LETTERS = re.compile(r"(.).\1")


def matches(pattern, string):
    return bool(re.search(pattern, string))


def is_nice_v1(string):
    def enough_vowels(string):
        return len(re.findall(VOWELS, string)) >= 3

    return enough_vowels(string) and matches(DOUBLE_LETTER, string) and not matches(DISALLOWED_STRINGS, string)


def is_nice_v2(string):
    return matches(DOUBLE_PAIR, string) and matches(SANDWICH_LETTERS, string)


assert is_nice_v1("ugknbfddgicrmopn")
assert is_nice_v1("aaa")
assert not is_nice_v1("jchzalrnumimnmhp")
assert not is_nice_v1("haegwjzuvuyypxyu")
assert not is_nice_v1("dvszwmarrgswjxmb")

assert is_nice_v2("qjhvhtzxzqqjkmpb")
assert is_nice_v2("xxyxx")
assert not is_nice_v2("uurcxstgmygtbstg")
assert not is_nice_v2("ieodomkazucvgmuy")

STRINGS = []
with open("data/05.txt") as fh:
    STRINGS = fh.readlines()

print(sum([int(is_nice_v1(string.strip())) for string in STRINGS]))
print(sum([int(is_nice_v2(string.strip())) for string in STRINGS]))
