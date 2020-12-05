""" day 9: all in a single night """
# note: greedy algorithm works because each location is connected to each other location

import re
import attr

from copy import deepcopy


@attr.s
class Location:
    name: str = attr.ib()
    distances = attr.ib(factory=list)  # looks like [(dest, dist), (dest, dist)..]

    def nearest(self):
        for loc in sorted(self.distances, key=lambda x: x[1]):
            yield loc

    def furthest(self):
        for loc in sorted(self.distances, key=lambda x: x[1], reverse=True):
            yield loc


def parse_distances(distances):
    pattern = re.compile(r"^([A-z]+) to ([A-z]+) = ([0-9]+)$")
    locations = {}
    for line in distances.split("\n"):
        if not line:
            continue

        match = re.match(pattern, line.strip())
        if not match:
            raise Exception(f"Line does not conform to pattern {pattern}: {line}")

        origin, destination, distance = match.groups()
        if origin not in locations:
            locations[origin] = Location(origin)
        if destination not in locations:
            locations[destination] = Location(destination)

        locations[origin].distances.append((destination, int(distance)))
        locations[destination].distances.append((origin, int(distance)))

    return locations


def shortest_distance_from(origin, locations):
    total_distance = 0
    location = locations.pop(origin)

    while len(locations) > 0:
        for nearest_location in location.nearest():
            if nearest_location[0] in locations.keys():
                break
        else:
            raise Exception("No unvisited nearby location found")

        total_distance += nearest_location[1]
        location = locations.pop(nearest_location[0])

    return total_distance


def shortest_distance(distances):
    locations = parse_distances(distances)
    min_distance = None

    for origin in locations:
        distance = shortest_distance_from(origin, deepcopy(locations))
        if not min_distance or distance < min_distance:
            min_distance = distance

    return min_distance


def furthest_distance_from(origin, locations):
    total_distance = 0
    location = locations.pop(origin)

    while len(locations) > 0:
        for furthest_location in location.furthest():
            if furthest_location[0] in locations.keys():
                break
        else:
            raise Exception("No unvisited nearby location found")

        total_distance += furthest_location[1]
        location = locations.pop(furthest_location[0])

    return total_distance


def furthest_distance(distances):
    locations = parse_distances(distances)
    max_distance = None

    for origin in locations:
        distance = furthest_distance_from(origin, deepcopy(locations))
        if not max_distance or distance > max_distance:
            max_distance = distance

    return max_distance


TEST_DISTANCES = """
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"""

assert shortest_distance(TEST_DISTANCES) == 605
assert furthest_distance(TEST_DISTANCES) == 982

DISTANCES = ""
with open("data/09.txt") as fh:
    DISTANCES = fh.read()

assert shortest_distance(DISTANCES) == 251
assert furthest_distance(DISTANCES) == 898
