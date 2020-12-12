""" day 18: like a gif for your yard """
from copy import deepcopy
from enum import Enum
from itertools import product

import attr


class Light(Enum):
    ON = "#"
    OFF = "."


@attr.s
class Grid:
    lights = attr.ib()
    corner_lights_stuck = attr.ib()

    def __attrs_post_init__(self):
        if self.corner_lights_stuck:
            for x, y in product([0, -1], [0, -1]):
                self.lights[y][x] = Light.ON

    @property
    def max_x(self):
        return len(self.lights[0])

    @property
    def max_y(self):
        return len(self.lights)

    def get(self, x, y):
        if x < 0 or y < 0:
            return None

        try:
            return self.lights[y][x]
        except IndexError:
            return None

    def neighbors(self, x, y):
        for x_step in [-1, 0, 1]:
            for y_step in [-1, 0, 1]:
                if x_step == 0 and y_step == 0:
                    continue

                if neighbor := self.get(x + x_step, y + y_step):
                    yield neighbor

    def _step(self):
        new_lights = deepcopy(self.lights)
        for y in range(self.max_y):
            for x in range(self.max_x):
                if self.corner_lights_stuck:
                    if y in [0, self.max_y - 1] and x in [0, self.max_x - 1]:
                        continue

                curr_light = self.get(x, y)
                on_neighbors = sum([int(n == Light.ON) for n in self.neighbors(x, y)])

                if curr_light == Light.ON and on_neighbors not in [2, 3]:
                    new_lights[y][x] = Light.OFF
                if curr_light == Light.OFF and on_neighbors == 3:
                    new_lights[y][x] = Light.ON

        self.lights = new_lights

    def step(self, n=1):
        for _ in range(n):
            self._step()

    def iter_all(self):
        for row in self.lights:
            for light in row:
                yield light

    def lights_on(self):
        return sum([int(l == Light.ON) for l in self.iter_all()])

    def __str__(self):
        return "\n".join(["".join([l.value for l in row]) for row in self.lights])

    @classmethod
    def init_from_str(cls, grid_str, corner_lights_stuck=False):
        return cls([[Light(x) for x in line.strip()] for line in grid_str.split("\n") if line], corner_lights_stuck)


TEST_LIGHT_STR = """
    .#.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####..
"""

TEST_LIGHT_STUCK_STR = """
    ##.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####.#
"""

TEST_LIGHTS = Grid.init_from_str(TEST_LIGHT_STR)
TEST_LIGHTS.step(4)
assert TEST_LIGHTS.lights_on() == 4

TEST_LIGHTS_STUCK = Grid.init_from_str(TEST_LIGHT_STUCK_STR, corner_lights_stuck=True)
TEST_LIGHTS_STUCK.step(5)
assert TEST_LIGHTS_STUCK.lights_on() == 17


with open("data/18.txt") as fh:
    LIGHT_STR = fh.read()

LIGHTS = Grid.init_from_str(LIGHT_STR)
LIGHTS.step(100)
print(LIGHTS.lights_on())

LIGHTS_STUCK = Grid.init_from_str(LIGHT_STR, corner_lights_stuck=True)
LIGHTS_STUCK.step(100)
print(LIGHTS_STUCK.lights_on())
