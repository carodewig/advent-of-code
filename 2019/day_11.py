"""
day 11: space police
"""

from enum import Enum
import attr

from day_05 import IntcodeComputer, test_intcode_computer


class Color(Enum):
    BLACK = "."
    WHITE = "#"

    @classmethod
    def for_val(cls, val):
        if val == 1:
            return cls.WHITE

        return cls.BLACK


@attr.s(slots=True)
class PainterRobot:
    intcode_computer = attr.ib()
    area_to_paint = attr.ib()

    robot_location = attr.ib(init=False)

    @classmethod
    def parse_area_str_to_list(cls, area_str):
        area_list = []
        for line in area_str.split("\n"):
            if not line:
                continue

            area_list.append([Color(x) for x in line.strip()])

        return area_list

    @classmethod
    def init_from_strs(cls, program_list, area_str):
        return PainterRobot(IntcodeComputer(program_list), cls.parse_area_str_to_list(area_str))

    @classmethod
    def init_from_files(cls, program_file, area_file):
        with open(area_file) as area_io:
            return PainterRobot(IntcodeComputer.init_from_file(program_file), cls.parse_area_str_to_list(area_io.read()))
