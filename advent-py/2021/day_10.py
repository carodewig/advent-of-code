""" day 10: syntax scoring """

from queue import LifoQueue
from collections import namedtuple

BracketSet = namedtuple("BracketSet", ["left", "right", "corrupt_points", "complete_points"])
BRACKET_SETS = [
    BracketSet("(", ")", 3, 1),
    BracketSet("[", "]", 57, 2),
    BracketSet("{", "}", 1197, 3),
    BracketSet("<", ">", 25137, 4),
]


def find_bracket_set(char):
    for bracket in BRACKET_SETS:
        if bracket.left == char or bracket.right == char:
            return bracket


def get_corrupt_score(line):
    q = LifoQueue()
    for char in line.strip():
        bracket_set = find_bracket_set(char)

        # opening bracket
        if char == bracket_set.left:
            q.put(char)

        # closing bracket
        elif char == bracket_set.right:
            if q.empty():
                return

            opening_char = q.get()
            if opening_char != bracket_set.left:
                return bracket_set.corrupt_points


def get_total_corrupt_score(lines):
    return sum(filter(None, [get_corrupt_score(line) for line in lines.split("\n")]))


def is_corrupted(line):
    return get_corrupt_score(line) is not None


def autocomplete(line):
    q = LifoQueue()
    for char in line.strip():
        bracket_set = find_bracket_set(char)

        # opening bracket
        if char == bracket_set.left:
            q.put(char)

        # closing bracket
        elif char == bracket_set.right:
            if q.empty():
                return

            q.get()

    while not q.empty():
        opening_char = q.get()

        bracket_set = find_bracket_set(opening_char)
        if opening_char == bracket_set.left:
            yield bracket_set.right


def get_autocomplete_score(line):
    if is_corrupted(line):
        return None

    total = None
    for closing_char in autocomplete(line):
        if total is None:
            total = 0

        total *= 5

        bracket_set = find_bracket_set(closing_char)
        if closing_char == bracket_set.right:
            total += bracket_set.complete_points

    return total


def best_score(lines):
    scores = sorted(list(filter(None, [get_autocomplete_score(line) for line in lines.split("\n")])))
    return scores[len(scores) // 2]


TEST_LINES = """
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"""

assert get_total_corrupt_score(TEST_LINES) == 26397
assert best_score(TEST_LINES) == 288957

with open("data/day_10.txt") as f:
    LINES = f.read()

assert get_total_corrupt_score(LINES) == 294195
assert best_score(LINES) == 3490802734
