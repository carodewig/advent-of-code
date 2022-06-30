""" day 17: conway cubes """

from enum import Enum
from copy import copy
from collections import defaultdict, namedtuple


class Cube(Enum):
    ACTIVE = "#"
    INACTIVE = "."


Location3d = namedtuple("Location3d", ["x", "y", "z"])
Location4d = namedtuple("Location4d", ["x", "y", "z", "w"])


def neighbors_3d(location):
    x, y, z = location
    for delx in [-1, 0, 1]:
        for dely in [-1, 0, 1]:
            for delz in [-1, 0, 1]:
                if delx == dely == delz == 0:
                    continue
                yield Location3d(x + delx, y + dely, z + delz)


def neighbors_4d(location):
    x, y, z, w = location
    for delx in [-1, 0, 1]:
        for dely in [-1, 0, 1]:
            for delz in [-1, 0, 1]:
                for delw in [-1, 0, 1]:
                    if delx == dely == delz == delw == 0:
                        continue
                    yield Location4d(x + delx, y + dely, z + delz, w + delw)


def empty_state():
    return defaultdict(lambda: Cube.INACTIVE)


def all_locations(initial_locations, neighbors_fn):
    processed_locations = set()

    for location in initial_locations:
        if location not in processed_locations:
            processed_locations.add(location)
            yield location

        for n_location in neighbors_fn(location):
            if n_location not in processed_locations:
                processed_locations.add(n_location)
                yield n_location


def step_location(location, initial_state, neighbors_fn):
    active_neighbors = sum([1 if initial_state[loc] == Cube.ACTIVE else 0 for loc in neighbors_fn(location)])

    if initial_state[location] == Cube.ACTIVE:
        if active_neighbors in [2, 3]:
            return Cube.ACTIVE
        return Cube.INACTIVE

    if active_neighbors == 3:
        return Cube.ACTIVE
    return Cube.INACTIVE


def step(initial_state, neighbors_fn):
    state = empty_state()

    for location in all_locations(list(initial_state.keys()), neighbors_fn):
        if step_location(location, initial_state, neighbors_fn) == Cube.ACTIVE:
            state[location] = Cube.ACTIVE

    return state


def total_active_cubes(state, cycles, neighbors_fn):
    for _ in range(cycles):
        state = step(state, neighbors_fn)

    return sum([1 if s == Cube.ACTIVE else 0 for s in state.values()])


def parse_state_3d(state_str):
    # top left corner = (0, 0, 0)
    state = empty_state()

    for (y, row) in enumerate(state_str.split("\n")):
        for (x, s) in enumerate(row.strip()):
            state[Location3d(x, y, 0)] = Cube(s)

    return state


def parse_state_4d(state_str):
    # top left corner = (0, 0, 0, 0)
    state = empty_state()

    for (y, row) in enumerate(state_str.split("\n")):
        for (x, s) in enumerate(row.strip()):
            state[Location4d(x, y, 0, 0)] = Cube(s)

    return state


def total_active_cubes_3d(state_str, cycles):
    return total_active_cubes(parse_state_3d(state_str), cycles, neighbors_3d)


def total_active_cubes_4d(state_str, cycles):
    return total_active_cubes(parse_state_4d(state_str), cycles, neighbors_4d)


TEST_STATE_STR = """
.#.
..#
###
"""

STATE_STR = ""
with open("data/17.txt") as fh:
    STATE_STR = fh.read()


assert len(list(neighbors_3d(Location3d(0, 0, 0)))) == 26
assert total_active_cubes_3d(TEST_STATE_STR, 6) == 112
assert total_active_cubes_3d(STATE_STR, 6) == 317

assert len(list(neighbors_4d(Location4d(0, 0, 0, 0)))) == 80
assert total_active_cubes_4d(TEST_STATE_STR, 6) == 848
print(total_active_cubes_4d(STATE_STR, 6))
