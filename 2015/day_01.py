""" day 1: not quite lisp """


def parse_instructions(instructions):
    floor = 0
    for instruction in instructions:
        if instruction == "(":
            floor += 1
        elif instruction == ")":
            floor -= 1

        yield floor

def entered_basement(instructions):
    for position, floor in enumerate(parse_instructions(instructions), start=1):
        if floor == -1:
            return position

def last_floor(instructions):
    return list(parse_instructions(instructions))[-1]

assert last_floor("(())") == 0
assert last_floor("()()") == 0
assert last_floor("(((") == 3
assert last_floor("(()(()(") == 3
assert last_floor("))(((((") == 3
assert last_floor("())") == -1
assert last_floor("))(") == -1
assert last_floor(")))") == -3
assert last_floor(")())())") == -3

assert entered_basement(")") == 1
assert entered_basement("()())") == 5

INSTRUCTIONS = ""
with open("data/01.txt") as fh:
    INSTRUCTIONS = fh.readline().strip()

assert last_floor(INSTRUCTIONS) == 232
assert entered_basement(INSTRUCTIONS) == 1783
