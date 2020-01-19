"""
day 10: monitoring station
https://adventofcode.com/2019/day/10
"""

import attr
import math

from enum import Enum

def can_divide(x):
    if x == 0:
        return True
    if x == 1:
        return False

    for y in range(2, x+1):
        if x % y == 0:
            return True

    return False

def get_direction(loc, tgt):
    if loc == tgt:
        return 0

    return int((tgt-loc) / abs(tgt-loc))

def get_steps(loc_x, loc_y, tgt_x, tgt_y):
    dist_x, dist_y = abs(loc_x - tgt_x), abs(loc_y - tgt_y)

    while can_divide(dist_x) and can_divide(dist_y):
        divs_this_round = 0

        for div in range(2, max(dist_x, dist_y)+1):
            if dist_x % div == 0 and dist_y % div == 0:
                dist_x, dist_y = int(dist_x / div), int(dist_y / div)
                divs_this_round += 1

        # divide until we can't anymore
        if divs_this_round == 0:
            break

    return (dist_x * get_direction(loc_x, tgt_x), dist_y * get_direction(loc_y, tgt_y))


class State(Enum):
    ASTEROID = '#'
    EMPTY = '.'

@attr.s(slots=True)
class AsteroidField:
    asteroid_map = attr.ib()

    def gen_locations(self):
        for loc_y in range(len(self.asteroid_map)):
            for loc_x in range(len(self.asteroid_map[loc_y])):
                yield (loc_x, loc_y)

    def get_at_location(self, loc_x, loc_y):
        if len(self.asteroid_map) <= loc_y:
            return State.EMPTY

        if len(self.asteroid_map[loc_y]) <= loc_x:
            return State.EMPTY

        return self.asteroid_map[loc_y][loc_x]

    def view_is_blocked(self, loc_x, loc_y, tgt_x, tgt_y):
        step_x, step_y = get_steps(loc_x, loc_y, tgt_x, tgt_y)

        def take_step(loc_x, loc_y):
            return (loc_x + step_x, loc_y + step_y)

        loc_x, loc_y = take_step(loc_x, loc_y)

        while loc_x != tgt_x or loc_y != tgt_y:
            if self.get_at_location(loc_x, loc_y) is State.ASTEROID:
                return True

            loc_x, loc_y = take_step(loc_x, loc_y)

        return False

    def visible_asteroids_from_location(self, loc_x, loc_y):
        asteroid_locations = []

        for (tgt_x, tgt_y) in self.gen_locations():
            # skip asteroid at current location
            if tgt_x == loc_x and tgt_y == loc_y:
                continue

            if self.get_at_location(tgt_x, tgt_y) is State.ASTEROID and not self.view_is_blocked(loc_x, loc_y, tgt_x, tgt_y):
                asteroid_locations.append((tgt_x, tgt_y))

        return asteroid_locations

    def count_asteroids_from_location(self, loc_x, loc_y):
        if self.get_at_location(loc_x, loc_y) is not State.ASTEROID:
            return -1

        return len(self.visible_asteroids_from_location(loc_x, loc_y))

    def best_station_location(self):
        max_asteroids_seen = 0
        best_x, best_y = 0, 0

        for (loc_x, loc_y) in self.gen_locations():
            asteroids = self.count_asteroids_from_location(loc_x, loc_y)
            if asteroids > max_asteroids_seen:
                max_asteroids_seen = asteroids
                best_x, best_y = loc_x, loc_y

        return best_x, best_y, max_asteroids_seen

    @classmethod
    def init_from_str(cls, asteroid_map_str):
        asteroid_map = []
        for line in asteroid_map_str.split("\n"):
            if not line:
                continue

            asteroid_map.append([State(x) for x in line.strip()])

        return AsteroidField(asteroid_map)

    @classmethod
    def init_from_file(cls, asteroid_map_file):
        with open(asteroid_map_file) as asteroid_map_io:
            return cls.init_from_str(asteroid_map_io.read())

    @classmethod
    def unit_test(cls):
        assert cls.init_from_str(".#..#\n.....\n#####\n....#\n...##").best_station_location() == (3, 4, 8)
        assert cls.init_from_str("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####").best_station_location() == (5, 8, 33)
        assert cls.init_from_str("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.").best_station_location() == (1, 2, 35)
        assert cls.init_from_str(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..").best_station_location() == (6, 3, 41)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").best_station_location() == (11, 13, 210)



AsteroidField.unit_test()
FIELD = AsteroidField.init_from_file("data/10.txt")
print(FIELD.best_station_location())
