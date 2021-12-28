""" day 9: smoke basin """

from collections import namedtuple
from queue import Queue
from functools import reduce

Location = namedtuple("Location", ["x", "y"])


class HeightMap:
    def __init__(self, heightmap):
        self.heightmap = heightmap

    def adjacent_locations(self, loc):
        for (delx, dely) in [(0, -1), (0, 1), (1, 0), (-1, 0)]:
            nearby = Location(loc.x + delx, loc.y + dely)
            if nearby in self.heightmap:
                yield nearby

    def low_points(self):
        for loc in self.heightmap:
            is_lowest = True
            for nearby in self.adjacent_locations(loc):
                if self.heightmap[nearby] <= self.heightmap[loc]:
                    is_lowest = False
                    break

            if is_lowest:
                yield loc

    def basin_sizes(self):
        # each basin has a single low point, so start from there and search
        # outward to find the boundary
        for low_point in self.low_points():
            basin_locs = set()
            loc_queue = Queue()
            loc_queue.put(low_point)

            while not loc_queue.empty():
                loc = loc_queue.get()
                basin_locs.add(loc)

                for near in self.adjacent_locations(loc):
                    if near in basin_locs or self.heightmap[near] == 9:
                        continue

                    loc_queue.put(near)

            yield len(basin_locs)

    def total_risk_of_low_points(self):
        return sum([1 + self.heightmap[loc] for loc in self.low_points()])

    def largest_basins_rating(self):
        largest_basins = sorted(list(self.basin_sizes()), reverse=True)[0:3]
        return reduce(lambda x, y: x * y, largest_basins, 1)

    @classmethod
    def from_str(cls, map_str):
        heightmap = {}
        map_lines = [line.strip() for line in map_str.split("\n") if line]

        for (y, line) in enumerate(map_lines):
            for (x, height) in enumerate(line):
                heightmap[Location(x, y)] = int(height)

        return cls(heightmap)


TEST_MAP = """
2199943210
3987894921
9856789892
8767896789
9899965678
"""

assert len(list(HeightMap.from_str(TEST_MAP).low_points())) == 4
assert HeightMap.from_str(TEST_MAP).total_risk_of_low_points() == 15
assert HeightMap.from_str(TEST_MAP).largest_basins_rating() == 1134


with open("data/day_09.txt") as f:
    MAP = f.read()

print(HeightMap.from_str(MAP).total_risk_of_low_points())
print(HeightMap.from_str(MAP).largest_basins_rating())
