""" day 9: encoding error """

from day_01 import two_entries_that_sum_to


def sums_to(goal, numbers):
    return two_entries_that_sum_to(goal, numbers) is not None


def xmas(numbers, preamble_length):
    for index in range(preamble_length, len(numbers)):
        goal, sublist = numbers[index], numbers[index - preamble_length : index]
        if not sums_to(goal, sublist):
            yield goal


def first_invalid_number(numbers, preamble_length):
    return next(xmas(numbers, preamble_length))


def encryption_weakness(numbers, preamble_length):
    target_value = first_invalid_number(numbers, preamble_length)

    for i in range(len(numbers)):
        for j in range(len(numbers)):
            subset = numbers[i : j + 1]

            if sum(subset) == target_value:
                return min(subset) + max(subset)

            if sum(subset) > target_value:
                break


TEST_EXAMPLES = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]
assert first_invalid_number(TEST_EXAMPLES, 5) == 127
assert encryption_weakness(TEST_EXAMPLES, 5) == 62


with open("data/09.txt") as fh:
    EXAMPLES = [int(x.strip()) for x in fh.readlines()]

assert first_invalid_number(EXAMPLES, 25) == 32321523
assert encryption_weakness(EXAMPLES, 25) == 4794981
