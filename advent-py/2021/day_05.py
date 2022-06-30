""" day 5: hydrothermal venture """


from collections import defaultdict, namedtuple

Location = namedtuple("Location", ["x", "y"])


class VentLine:
    def __init__(self, point1, point2):
        self.point1 = point1
        self.point2 = point2

    @property
    def diagonal(self):
        return self.point1.x != self.point2.x and self.point1.y != self.point2.y

    @staticmethod
    def _direction(p1, p2):
        try:
            return int((p2 - p1) / abs(p2 - p1))
        except ZeroDivisionError:
            return 0

    def covered_locations(self):
        x = self.point1.x
        y = self.point1.y

        xdir = self._direction(self.point1.x, self.point2.x)
        ydir = self._direction(self.point1.y, self.point2.y)

        # point 1 and everything up to point 2
        while x != self.point2.x or y != self.point2.y:
            yield Location(x, y)
            x += xdir
            y += ydir

        # point 2
        yield Location(x, y)

    @classmethod
    def init_from_str(cls, desc):
        point1_str = desc.split()[0]
        point2_str = desc.split()[2]

        return cls(
            Location(*[int(x) for x in point1_str.strip().split(",")]),
            Location(*[int(x) for x in point2_str.strip().split(",")]),
        )


def parse_lines(vents_str):
    lines = []
    for line in vents_str.split("\n"):
        if not line:
            continue

        lines.append(VentLine.init_from_str(line.strip()))

    return lines


def count_dangerous_areas(vent_lines, include_diagonals=False):
    crosses = defaultdict(int)

    for vent_line in vent_lines:
        if not include_diagonals and vent_line.diagonal:
            continue

        for loc in vent_line.covered_locations():
            crosses[loc] += 1

    return len([loc for loc in crosses if crosses[loc] > 1])


TEST_LINES = """
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"""

TEST_VENT_LINES = parse_lines(TEST_LINES)
assert count_dangerous_areas(TEST_VENT_LINES) == 5
assert count_dangerous_areas(TEST_VENT_LINES, include_diagonals=True) == 12


with open("data/day_05.txt") as f:
    VENT_LINES = parse_lines(f.read())

print(count_dangerous_areas(VENT_LINES))
print(count_dangerous_areas(VENT_LINES, include_diagonals=True))
