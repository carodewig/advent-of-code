""" day 7: the treachery of whales """

import math


def fuel_part1(steps):
    return abs(steps)


def fuel_part2(steps):
    return sum(range(abs(steps) + 1))


def fuel_to_align(crabs, target_pos, fuel_fn):
    return sum([fuel_fn(crab - target_pos) for crab in crabs])


# this feels like a binary search problem but I'll start with brute force
# worked in 310ms for part 1, 22s for part2
def min_fuel_to_align_brute_force(crabs, fuel_fn):
    min_fuel = math.inf
    min_pos = min(crabs)
    max_pos = max(crabs)

    for pos in range(min_pos, max_pos + 1):
        fuel = fuel_to_align(crabs, pos, fuel_fn)
        min_fuel = min(min_fuel, fuel)

    return min_fuel


# binary search is the way to go, took total runtime down to 251ms!
def min_fuel_to_align_binary_search(crabs, fuel_fn):
    def _fuel_to_align(pos):
        return fuel_to_align(crabs, pos, fuel_fn)

    def search(left_pos, right_pos):
        diff = right_pos - left_pos
        if diff == 0:
            return _fuel_to_align(left_pos)
        elif diff == 1:
            return min(_fuel_to_align(left_pos), _fuel_to_align(right_pos))

        left_fuel = _fuel_to_align(left_pos)
        right_fuel = _fuel_to_align(right_pos)

        mid_pos = left_pos + int(diff / 2)

        if left_fuel < right_fuel:
            return search(left_pos, mid_pos)
        elif left_fuel > right_fuel:
            return search(mid_pos, right_pos)
        else:
            return min(search(left_pos, mid_pos), search(mid_pos, right))

    return search(min(crabs), max(crabs))


def min_fuel_to_align(crabs, fuel_fn):
    return min_fuel_to_align_binary_search(crabs, fuel_fn)


def parse_crabs(crab_str):
    return [int(x) for x in crab_str.strip().split(",")]


TEST_CRABS = parse_crabs("16,1,2,0,4,2,7,1,2,14")
assert min_fuel_to_align(TEST_CRABS, fuel_part1) == 37
assert min_fuel_to_align(TEST_CRABS, fuel_part2) == 168

with open("data/day_07.txt") as f:
    CRABS = parse_crabs(f.read())

assert min_fuel_to_align(CRABS, fuel_part1) == 344297
assert min_fuel_to_align(CRABS, fuel_part2) == 97164301
