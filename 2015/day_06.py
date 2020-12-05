""" day 6: probably a fire hazard """

import attr


@attr.s
class LightDisplay:
    grid = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.grid = [[0 for _ in range(1000)] for _ in range(1000)]

    def turn_on(self, x, y):
        self.grid[y][x] += 1

    def turn_off(self, x, y):
        self.grid[y][x] -= 1
        self.grid[y][x] = max(0, self.grid[y][x])

    def toggle(self, x, y):
        self.grid[y][x] += 2

    def parse_instruction(self, string):
        start_loc, end_loc, func = None, None, None

        if string.startswith("turn on"):
            start_loc, end_loc = string.split()[2], string.split()[4]
            func = self.turn_on
        elif string.startswith("turn off"):
            start_loc, end_loc = string.split()[2], string.split()[4]
            func = self.turn_off
        else:
            start_loc, end_loc = string.split()[1], string.split()[3]
            func = self.toggle

        start_x, start_y = [int(x) for x in start_loc.split(",")]
        end_x, end_y = [int(x) for x in end_loc.split(",")]
        for y in range(start_y, end_y + 1):
            for x in range(start_x, end_x + 1):
                func(x, y)

    def brightness(self):
        return sum([sum(row) for row in self.grid])

    @classmethod
    def init(cls, instructions):
        display = cls()
        for instruction in instructions:
            display.parse_instruction(instruction)

        return display


DISPLAY = LightDisplay()
with open("data/06.txt") as fh:
    for line in fh:
        DISPLAY.parse_instruction(line)

print(DISPLAY.brightness())
