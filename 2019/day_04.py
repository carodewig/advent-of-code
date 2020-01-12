"""
day 4: secure container
https://adventofcode.com/2019/day/4
"""

from collections import defaultdict


def is_increasing(password):
    return not any([password[i] > password[i + 1] for i in range(len(password) - 1)])


def has_pairs(password):
    counts = defaultdict(int)
    for i in password:
        counts[i] += 1

    return any(x == 2 for x in counts.values())


def meets_criteria(password):
    if len(password) != 6:
        return False

    if not is_increasing([int(x) for x in password]):
        return False

    if not has_pairs(password):
        return False

    return True


# SLOW -- works, but first pass solution only
def count_valid_passwords_SLOW(lower_bound, upper_bound):
    valid_count = 0
    for password in range(int(lower_bound), int(upper_bound) + 1):
        if meets_criteria(str(password)):
            valid_count += 1
    return valid_count


def str_to_list(s):
    return [int(x) for x in s]


def list_to_int(l):
    return int("".join([str(x) for x in l]))


def _count_valid_passwords_recursive(prefix, lower_bound, upper_bound):
    if len(prefix) == 6:
        return int(meets_criteria(prefix))

    if prefix:
        if list_to_int(prefix) < list_to_int(lower_bound[: len(prefix)]):
            return 0
        if list_to_int(prefix) > list_to_int(upper_bound[: len(prefix)]):
            return 0

    valid_count = 0
    for next_digit in range(10)[::-1]:
        if prefix and prefix[-1] > next_digit:
            return valid_count

        valid_count += _count_valid_passwords_recursive(prefix + [next_digit], lower_bound, upper_bound)

    return valid_count


def count_valid_passwords(lower_bound, upper_bound):
    return _count_valid_passwords_recursive([], str_to_list(lower_bound), str_to_list(upper_bound))


assert meets_criteria("112233")
assert meets_criteria("111122")
assert not meets_criteria("123444")

print(count_valid_passwords("138241", "674034"))  # 1890
