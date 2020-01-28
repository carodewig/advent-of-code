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
    backup_asteroid_map = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.backup_asteroid_map = list(self.asteroid_map)

    def reset(self):
        self.asteroid_map = list(self.backup_asteroid_map)

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

    def vaporize_one_asteroid(self, loc_x, loc_y):
        if len(self.asteroid_map) <= loc_y:
            return

        if len(self.asteroid_map[loc_y]) <= loc_x:
            return

        self.asteroid_map[loc_y][loc_x] = State.EMPTY

    def sort_for_vaporizer(self, station_loc):
        def sort_for_vaporizer_station_at_origin(asteroid_loc_raw):
            asteroid_loc_x, asteroid_loc_y = (asteroid_loc_raw[0] - station_loc[0], station_loc[1] - asteroid_loc_raw[1])

            # get arctan
            val = math.atan2(asteroid_loc_y, asteroid_loc_x)

            # goes from -pi to pi --> so sec1 = pi/2:0, sec2 = 0:-pi/2, sec3 = -pi/2:-pi, sec4=pi:pi/2
            # need to subtract 2pi from sec 4 so that it goes last
            if asteroid_loc_x < 0 and asteroid_loc_y >= 0:
                val -= 2*math.pi

            return val

        return sort_for_vaporizer_station_at_origin

    def vaporize(self, nth_to_be_vaporized=None):
        station_x, station_y, num_asteroids_currently_visible = self.best_station_location()
        visible_asteroids = self.visible_asteroids_from_location(station_x, station_y)

        index = 1

        cmp_fn = self.sort_for_vaporizer((station_x, station_y))

        while num_asteroids_currently_visible > 0:
            for asteroid_loc in sorted(visible_asteroids, key=cmp_fn, reverse=True):
                self.vaporize_one_asteroid(*asteroid_loc)
                
                if nth_to_be_vaporized is not None and nth_to_be_vaporized == index:
                    self.reset()
                    return asteroid_loc

                index += 1

            visible_asteroids = self.visible_asteroids_from_location(station_x, station_y)
            num_asteroids_currently_visible = len(visible_asteroids)

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

        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=1) == (11, 12)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=2) == (12, 1)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=3) == (12, 2)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=10) == (12, 8)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=20) == (16, 0)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=50) == (16, 9)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=100) == (10, 16)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=199) == (9, 6)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=200) == (8, 2)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=201) == (10, 9)
        assert cls.init_from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##").vaporize(nth_to_be_vaporized=299) == (11, 1)


AsteroidField.unit_test()
FIELD = AsteroidField.init_from_file("data/10.txt")
print(FIELD.best_station_location())
print(FIELD.vaporize(nth_to_be_vaporized=200))
