""" day 18: operation order """

from collections import deque


def isnum(char):
    try:
        int(char)
        return True
    except ValueError:
        return False


def isop(char):
    return char in ["+", "*"]


def peekleft(stack):
    try:
        tmp = stack.popleft()
        stack.appendleft(tmp)
    except IndexError:
        return None

    return tmp


# assumes all parens have been filtered out
def compute_part1(calc_stack):
    while True:
        num1 = calc_stack.popleft()
        if not peekleft(calc_stack):
            return num1

        op = calc_stack.popleft()
        num2 = calc_stack.popleft()

        if op == "+":
            result = num1 + num2
        else:
            result = num1 * num2

        calc_stack.appendleft(result)


# assumes all parens have been filtered out
def compute_part2(calc_stack):
    tmp_stack = deque()

    # addition first
    while peekleft(calc_stack):
        num1 = calc_stack.popleft()
        if not peekleft(calc_stack):
            tmp_stack.append(num1)
            break

        op = calc_stack.popleft()
        num2 = calc_stack.popleft()

        if op == "+":
            result = num1 + num2
            calc_stack.appendleft(result)
            continue

        tmp_stack.append(num1)
        tmp_stack.append(op)
        calc_stack.appendleft(num2)

    # then handle multiplication
    return compute_part1(tmp_stack)


def calculate(math_str, compute_fn):
    calc_stack = deque()
    math_str = math_str.replace("(", " ( ").replace(")", " ) ")

    for char in math_str.split(" "):
        if isnum(char):
            calc_stack.append(int(char))

        if isop(char) or char == "(":
            calc_stack.append(char)

        if char == ")":
            sub_stack = deque()
            while (subchar := calc_stack.pop()) != "(":
                sub_stack.appendleft(subchar)

            calc_stack.append(compute_fn(sub_stack))

    return compute_fn(calc_stack)


def calc_part1(math_str):
    return calculate(math_str, compute_part1)


def calc_part2(math_str):
    return calculate(math_str, compute_part2)


assert calc_part1("1 + 2 * 3 + 4 * 5 + 6") == 71
assert calc_part1("1 + (2 * 3) + (4 * (5 + 6))") == 51
assert calc_part1("2 * 3 + (4 * 5)") == 26
assert calc_part1("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 437
assert calc_part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 12240
assert calc_part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 13632

assert calc_part2("1 + 2 * 3 + 4 * 5 + 6") == 231
assert calc_part2("1 + (2 * 3) + (4 * (5 + 6))") == 51
assert calc_part2("2 * 3 + (4 * 5)") == 46
assert calc_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 1445
assert calc_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 669060
assert calc_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 23340

total = 0
with open("data/18.txt") as fh:
    for line in fh:
        total += calc_part1(line)

print(total)


total = 0
with open("data/18.txt") as fh:
    for line in fh:
        total += calc_part2(line)

print(total)
