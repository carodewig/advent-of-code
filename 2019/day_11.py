"""
day 11: space police
"""

from enum import Enum
import attr
import math

from day_05 import IntcodeComputer, test_intcode_computer


class Color(Enum):
    BLACK = 0
    WHITE = 1

    def to_str(self):
        if self.value:
            return "+"
        return " "


@attr.s(slots=True)
class Location:
    # grid indexed from top left
    loc_x = attr.ib()
    loc_y = attr.ib()
    orientation = attr.ib(default=0)

    def get(self):
        return (self.loc_x, self.loc_y)

    def turn(self, direction):
        if direction:
            # turn right
            self.orientation += 90
        else:
            # turn left
            self.orientation -= 90

    def step(self):
        angle = math.radians(self.orientation)

        self.loc_x += round(math.sin(angle))
        self.loc_y -= round(math.cos(angle))


@attr.s(slots=True)
class PainterRobot:
    intcode_computer = attr.ib()
    robot_location = attr.ib(init=False)
    area_to_paint = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.robot_location = Location(0, 0)
        self.area_to_paint = dict()

    def print_area(self):
        min_x = min([point[0] for point in self.area_to_paint.keys()])
        max_x = max([point[0] for point in self.area_to_paint.keys()])
        min_y = min([point[1] for point in self.area_to_paint.keys()])
        max_y = max([point[1] for point in self.area_to_paint.keys()])

        for y in range(min_y, max_y + 1):
            for x in range(min_x, max_x + 1):
                if (x, y) in self.area_to_paint:
                    print(self.area_to_paint[(x, y)].to_str(), end="")
                else:
                    print(Color.BLACK.to_str(), end="")
            print()

    def get_color(self):
        location = self.robot_location.get()
        if location in self.area_to_paint:
            return self.area_to_paint[location]

        return Color.BLACK

    def paint(self, color):
        self.area_to_paint[self.robot_location.get()] = color

    def run(self):
        panels_painted = set()

        while self.intcode_computer.is_alive():
            self.intcode_computer.pass_in(self.get_color().value)
            new_color = self.intcode_computer.parse_and_get_next_value()
            turn_dir = self.intcode_computer.parse_and_get_next_value()

            if new_color is None or turn_dir is None:
                break

            self.paint(Color(new_color))
            panels_painted.add(self.robot_location.get())

            self.robot_location.turn(turn_dir)
            self.robot_location.step()

        return len(panels_painted)

    @classmethod
    def init_from_file(cls, program_file):
        return PainterRobot(IntcodeComputer.init_from_file(program_file))

    @classmethod
    def init_from_mock(cls, program):
        return PainterRobot(MockIntcodeComputer(program))


@attr.s
class MockIntcodeComputer:
    program = attr.ib()

    def is_alive(self):
        return len(self.program)

    def parse_and_get_next_value(self):
        return self.program.pop(0)

    def pass_in(self, value):
        pass


MOCKED_ROBOT = PainterRobot.init_from_mock([1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0])
assert MOCKED_ROBOT.run() == 6

ROBOT = PainterRobot.init_from_file("data/11.txt")

# set up starting white panel
ROBOT.paint(Color.WHITE)
ROBOT.run()
ROBOT.print_area()
