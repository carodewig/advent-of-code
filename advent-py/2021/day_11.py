""" day 11: dumbo octobus """

import itertools
from collections import namedtuple

Location = namedtuple("Location", ["x", "y"])


class Octopi:
    def __init__(self, grid):
        self.grid = grid

    def nearby(self, loc):
        for (delx, dely) in itertools.product(range(-1, 2), range(-1, 2)):
            if delx == 0 and dely == 0:
                continue

            nearby = Location(loc.x + delx, loc.y + dely)
            if nearby in self.grid:
                yield nearby

    @classmethod
    def from_str(cls, grid_str):
        grid_lst = [line.strip() for line in grid_str.strip().split("\n") if line]
        grid = {}

        for (y, line) in enumerate(grid_lst):
            for (x, energy) in enumerate(line):
                grid[Location(x, y)] = int(energy)

        return cls(grid)

    def step(self):
        for loc in self.grid:
            self.grid[loc] += 1

        flashed_locations = set()
        new_flashes = True

        while new_flashes:
            new_flashes = False
            for loc in self.grid:
                if loc in flashed_locations:
                    continue

                if self.grid[loc] > 9:
                    flashed_locations.add(loc)
                    new_flashes = True

                    for nearby_loc in self.nearby(loc):
                        self.grid[nearby_loc] += 1

        for loc in flashed_locations:
            self.grid[loc] = 0

        return len(flashed_locations)

    def total_flashes(self, num_steps):
        return sum([self.step() for _ in range(num_steps)])

    def first_synchronized_flash(self):
        step = 0
        while True:
            step += 1
            flashes = self.step()

            if flashes == len(self.grid):
                return step


TEST_STR = """
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""

with open("data/day_11.txt") as f:
    REAL_STR = f.read()


assert Octopi.from_str(TEST_STR).total_flashes(100) == 1656
assert Octopi.from_str(REAL_STR).total_flashes(100) == 1717

assert Octopi.from_str(TEST_STR).first_synchronized_flash() == 195
assert Octopi.from_str(REAL_STR).first_synchronized_flash() == 476
