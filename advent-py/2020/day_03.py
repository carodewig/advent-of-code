""" day 3: toboggan trajectory """

from collections import namedtuple
from enum import Enum
import attr

Location = namedtuple("Location", ["x", "y"])


class State(Enum):
    TREE = "#"
    OPEN = "."


@attr.s
class TobogganRun:
    tree_map = attr.ib()

    def tree(self, location):
        if location.y >= len(self.tree_map):
            return False

        tree_map_row = self.tree_map[location.y]
        return tree_map_row[location.x % len(tree_map_row)] == State.TREE

    def toboggo(self, right, down):
        location = Location(0, 0)
        trees_hit = 0

        while location.y < len(self.tree_map):
            location = Location(location.x + right, location.y + down)
            if self.tree(location):
                trees_hit += 1

        return trees_hit

    def toboggo_many_slopes(self, slopes):
        result = 1
        for slope in slopes:
            result *= self.toboggo(*slope)

        return result

    @classmethod
    def from_str(cls, map_str):
        return cls([[State(x) for x in line.strip()] for line in map_str.split() if len(line.strip())])

    def __str__(self):
        return "\n".join(["".join([x.value for x in row]) for row in self.tree_map])


TEST_MAP = """
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"""
SLOPES = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

TEST_RUN = TobogganRun.from_str(TEST_MAP)
assert TEST_RUN.toboggo(3, 1) == 7
assert TEST_RUN.toboggo_many_slopes(SLOPES) == 336

REAL_MAP = ""
with open("data/03.txt") as fh:
    REAL_MAP = fh.read()

REAL_RUN = TobogganRun.from_str(REAL_MAP)
assert REAL_RUN.toboggo(3, 1) == 257
assert REAL_RUN.toboggo_many_slopes(SLOPES) == 1744787392
