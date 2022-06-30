import re

CRITERIA = ("children", "cats", "samoyeds", "pomeranians", "akitas", "vizslas", "goldfish", "trees", "cars", "perfumes")


def criteria_match_part1(criteria, sue):
    for (criterion, value) in sue.items():
        if criteria[criterion] != value:
            return False

    return True


def criteria_match_part2(criteria, sue):
    for (criterion, value) in sue.items():
        if criterion in ["cats", "trees"]:
            if criteria[criterion] >= value:
                return False
            continue

        if criterion in ["pomeranians", "goldfish"]:
            if criteria[criterion] <= value:
                return False
            continue

        # some are still accurate
        if criteria[criterion] != value:
            return False

    return True


def parse_sues(sue_str):
    sues = {}

    for sue_line in sue_str.split("\n"):
        sue = {}

        index_pattern = r"Sue ([0-9]+):"
        if match := re.match(index_pattern, sue_line):
            sues[int(match.group(1))] = sue
        else:
            continue

        for criterion in CRITERIA:
            pattern = re.compile(f"({criterion}): ([0-9]+)")
            if match := re.search(pattern, sue_line):
                sue[match.group(1)] = int(match.group(2))

    return sues


SUES = {}
with open("data/16.txt") as fh:
    SUES = parse_sues(fh.read())

GIFTER = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}

for (INDEX, SUE) in SUES.items():
    if criteria_match_part1(GIFTER, SUE):
        print(INDEX)

for (INDEX, SUE) in SUES.items():
    if criteria_match_part2(GIFTER, SUE):
        print(INDEX)
