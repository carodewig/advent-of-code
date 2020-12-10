""" day 6 """


def parse_any_member(answers):
    total_yeses = 0
    group = []

    for line in answers.split("\n"):
        if line == "":
            total_yeses += len(set(group))
            group = []
        else:
            group += list(line)

    total_yeses += len(set(group))
    return total_yeses


def parse_all_members(answers):
    total_yeses = 0
    group = []

    for line in answers.split("\n"):
        if line == "":
            if group:
                total_yeses += len(set(group[0]).intersection(*group[1:]))
            group = []
        else:
            group.append(tuple(set(line)))

    total_yeses += len(set(group))
    return total_yeses


TEST_ANSWERS = """
abc

a
b
c

ab
ac

a
a
a
a

b"""

assert parse_any_member(TEST_ANSWERS) == 11
assert parse_all_members(TEST_ANSWERS) == 6

with open("data/06.txt") as fh:
    ANSWERS = fh.read()

assert parse_any_member(ANSWERS) == 6590
assert parse_all_members(ANSWERS) == 3288
