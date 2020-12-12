""" day 12: rain risk """

from collections import namedtuple

Location = namedtuple("Location", ["x", "y"])


def north(loc, dist):
    return Location(loc.x, loc.y + dist)
def east(loc, dist):
    return Location(loc.x + dist, loc.y)
def west(loc, dist):
    return Location(loc.x - dist, loc.y)
def south(loc, dist):
    return Location(loc.x, loc.y - dist)


def run_part1(directions_str):
    ship = Location(0, 0)
    ship_dir = 90 # 0-90-180-270 N-E-S-W

    for direction in directions_str.split("\n"):
        if not direction:
            continue

        action = direction[0]
        num = int(direction[1:])

        if action == "N":
            ship = north(ship, num)
        elif action == "S":
            ship = south(ship, num)
        elif action == "E":
            ship = east(ship, num)
        elif action == "W":
            ship = west(ship, num)
        elif action == "L":
            ship_dir -= num
            ship_dir = ship_dir % 360
        elif action == "R":
            ship_dir += num
            ship_dir = ship_dir % 360
        elif action == "F":
            if ship_dir == 0:
                ship = north(ship, num)
            elif ship_dir == 90:
                ship = east(ship, num)
            elif ship_dir == 180:
                ship = south(ship, num)
            elif ship_dir == 270:
                ship = west(ship, num)
            else:
                raise Exception(ship_dir)
        else:
            raise Exception(action)

    return abs(ship.x) + abs(ship.y)


def run_part2(directions_str):
    ship = Location(0, 0)
    waypoint = Location(10, 1)

    for direction in directions_str.split("\n"):
        if not direction:
            continue

        action = direction[0]
        num = int(direction[1:])

        if action == "N":
            waypoint = north(waypoint, num)
        elif action == "S":
            waypoint = south(waypoint, num)
        elif action == "E":
            waypoint = east(waypoint, num)
        elif action == "W":
            waypoint = west(waypoint, num)
        elif action == "L":
            for _ in range(int(num / 90)):
                delta_x = waypoint.x - ship.x
                delta_y = waypoint.y - ship.y
                waypoint = Location(ship.x - delta_y, ship.y + delta_x)
        elif action == "R":
            for _ in range(int(num / 90)):
                delta_x = waypoint.x - ship.x
                delta_y = waypoint.y - ship.y
                waypoint = Location(ship.x + delta_y, ship.y - delta_x)
        elif action == "F":
            for _ in range(num):
                delta_x = waypoint.x - ship.x
                delta_y = waypoint.y - ship.y
                ship = Location(ship.x + delta_x, ship.y + delta_y)
                waypoint = Location(waypoint.x + delta_x, waypoint.y + delta_y)
        else:
            raise Exception(action)

    return abs(ship.x) + abs(ship.y)

TEST_DIRECTIONS = """
F10
N3
F7
R90
F11
"""

assert run_part1(TEST_DIRECTIONS) == 25
assert run_part2(TEST_DIRECTIONS) == 286

with open("data/12.txt") as fh:
    DIRECTIONS = fh.read()

print(run_part1(DIRECTIONS))
print(run_part2(DIRECTIONS))
