""" day 1: report repair """

from math import floor


def two_entries_that_sum_to(goal, entries):
    if len(entries) < 2:
        return None

    entries_sorted = sorted(entries)

    index1 = 0
    index2 = len(entries_sorted) - 1

    while index1 < index2:
        result = entries_sorted[index1] + entries_sorted[index2]

        if result == goal:
            return entries_sorted[index1] * entries_sorted[index2]

        if result < goal:
            index1 += 1
        else:
            index2 -= 1

    return None


def three_entries_that_sum_to(goal, entries):
    entries_sorted = sorted(entries)

    if len(entries) < 3:
        return None

    for index in range(len(entries)):
        value = entries_sorted[index]
        if partial_result := two_entries_that_sum_to(goal - value, entries_sorted[index + 1 :]):
            return partial_result * value

    return None


def load_entries_from_text(filename):
    entries = []

    with open(filename) as fh:
        for line in fh:
            entries.append(int(line.strip()))

    return entries


TEST_SET = [1721, 979, 366, 299, 675, 1456]
assert two_entries_that_sum_to(2020, TEST_SET) == 514579
assert three_entries_that_sum_to(2020, TEST_SET) == 241861950

REAL_SET = load_entries_from_text("data/01.txt")
assert two_entries_that_sum_to(2020, REAL_SET) == 866436
assert three_entries_that_sum_to(2020, REAL_SET) == 276650720
