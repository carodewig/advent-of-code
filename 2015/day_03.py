""" day 3: perfectly spherical houses in a vacuum """

from collections import namedtuple

Location = namedtuple("Location", ["x", "y"])


def deliver_presents(directions):
    location = Location(0, 0)
    yield location

    for step in directions:
        if step == ">":
            location = Location(location.x + 1, location.y)
        elif step == "<":
            location = Location(location.x - 1, location.y)
        elif step == "^":
            location = Location(location.x, location.y + 1)
        elif step == "v":
            location = Location(location.x, location.y - 1)

        yield location

def unique_houses(directions):
    return len(set(list(deliver_presents(directions))))

def unique_houses_two_santas(directions):
    return len(set(list(deliver_presents(directions[::2])) + list(deliver_presents(directions[1::2]))))


assert unique_houses(">") == 2
assert unique_houses("^>v<") == 4
assert unique_houses("^v^v^v^v^v") == 2

assert unique_houses_two_santas("^v") == 3
assert unique_houses_two_santas("^>v<") == 3
assert unique_houses_two_santas("^v^v^v^v^v") == 11


DIRECTIONS = ""
with open("data/03.txt") as fh:
    DIRECTIONS = fh.readline().strip()


assert unique_houses(DIRECTIONS) == 2081
assert unique_houses_two_santas(DIRECTIONS) == 2341
