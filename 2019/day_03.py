"""
day 3: crossed wires
https://adventofcode.com/2019/day/3
"""

import sys

# probably should use a dict here (value = steps required) to make this faster
# current structure is [(x, y, steps_required)]
def points_hit_single_instruction(start_position, direction, distance):
    if direction == "R":
        return [(start_position[0] + x, start_position[1], start_position[2] + x) for x in range(1, distance + 1)]
    if direction == "L":
        return [(start_position[0] - x, start_position[1], start_position[2] + x) for x in range(1, distance + 1)]
    if direction == "U":
        return [(start_position[0], start_position[1] + x, start_position[2] + x) for x in range(1, distance + 1)]
    if direction == "D":
        return [(start_position[0], start_position[1] - x, start_position[2] + x) for x in range(1, distance + 1)]


def points_hit_by_wire(wire):
    points = []
    position = (0, 0, 0)

    for instruction in wire:
        direction, distance = instruction[0], int(instruction[1:].strip())
        points += points_hit_single_instruction(position, direction, distance)
        position = points[-1]

    return set(points)


def fewest_steps_to_intersection_point(wire1, wire2):
    min_steps = sys.maxsize
    points1 = points_hit_by_wire(wire1)
    points2 = points_hit_by_wire(wire2)

    intersections = set.intersection({(x[0], x[1]) for x in points1}, {(x[0], x[1]) for x in points2})

    for point in intersections:
        # first find in points1
        min_dist_p1 = sys.maxsize
        for p1 in points1:
            if p1[0:2] == point:
                min_dist_p1 = min(min_dist_p1, p1[2])

        # then find in points2
        min_dist_p2 = sys.maxsize
        for p2 in points2:
            if p2[0:2] == point:
                min_dist_p2 = min(min_dist_p2, p2[2])

        min_steps = min(min_steps, min_dist_p1 + min_dist_p2)

    return min_steps


def str_to_wire(wire_str):
    return wire_str.split(",")


def closest_point_from_strs(wire1_str, wire2_str):
    return fewest_steps_to_intersection_point(str_to_wire(wire1_str), str_to_wire(wire2_str))


def read_wire_strs_from_file(filename):
    with open(filename, "r") as f:
        return [line for line in f.readlines()]


# test cases
assert closest_point_from_strs("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83") == 610
assert closest_point_from_strs("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",) == 410

WIRES = read_wire_strs_from_file("data/03.txt")
if len(WIRES) > 1:
    print(closest_point_from_strs(WIRES[0], WIRES[1]))  # 399
