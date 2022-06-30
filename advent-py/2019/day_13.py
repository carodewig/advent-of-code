"""
day 13: care package
"""

import attr

from collections import defaultdict
from enum import Enum
from day_05 import IntcodeComputer, test_intcode_computer


class Tile(Enum):
    EMPTY = 0
    WALL = 1
    BLOCK = 2
    PADDLE = 3
    BALL = 4

    def __str__(self):
        views = [" ", "+", "#", "-", "o"]
        return views[self.value]


@attr.s
class ArcadeCabinet:
    intcode_computer = attr.ib()
    autoplay = attr.ib(default=False)

    screen = attr.ib(init=False)
    score = attr.ib(init=False, default=0)

    def __attrs_post_init__(self):
        self.screen = defaultdict(lambda x: Tile.EMPTY)

    def display(self):
        print(f"\n ~ ~ SCORE: {self.score} ~ ~\n")

        max_x = max([x[0] for x in self.screen.keys()]) + 1
        max_y = max([x[1] for x in self.screen.keys()]) + 1

        for y in range(max_y):
            for x in range(max_x):
                print(str(self.screen.get((x, y), Tile.EMPTY)), end="")
            print()

    def play_for_free(self):
        self.intcode_computer._set_value(0, 2)

    def _find_x(self, tile_value):
        for loc, tile in self.screen.items():
            if tile == tile_value:
                return loc[0]

        return None

    def auto_move_paddle(self):
        paddle_x = self._find_x(Tile.PADDLE)
        ball_x = self._find_x(Tile.BALL)

        if ball_x is not None and paddle_x is not None:
            if ball_x < paddle_x:
                self.intcode_computer.input_values = [-1]
            elif ball_x > paddle_x:
                self.intcode_computer.input_values = [1]
            else:
                self.intcode_computer.input_values = [0]

    def run(self):
        while self.intcode_computer.is_alive():
            x = self.intcode_computer.parse_and_get_next_value()
            y = self.intcode_computer.parse_and_get_next_value()
            tile_type = self.intcode_computer.parse_and_get_next_value()

            if any([tmp is None for tmp in [x, y, tile_type]]):
                break

            if (x, y) == (-1, 0):
                self.score = tile_type
                self.display()
                continue

            new_tile = Tile(tile_type)
            self.screen[(x, y)] = Tile(tile_type)

            if new_tile == Tile.PADDLE or new_tile == Tile.BALL:
                self.auto_move_paddle()

    @classmethod
    def init_from_file(cls, program_file):
        return ArcadeCabinet(IntcodeComputer.init_from_file(program_file))


PONG = ArcadeCabinet.init_from_file("data/13.txt")
PONG.play_for_free()
PONG.run()

# assert 193 == sum([1 if tile == Tile.BLOCK else 0 for tile in PONG.screen.values()])
# PONG.display()
