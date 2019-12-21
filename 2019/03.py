"""
day 3: crossed wires
https://adventofcode.com/2019/day/3
"""


def points_hit_single_instruction(start_position, direction, distance):
    if direction == "R":
        return [(start_position[0] + x, start_position[1]) for x in range(1, distance+1)]
    elif direction == "L":
        return [(start_position[0] - x, start_position[1]) for x in range(1, distance+1)]
    elif direction == "U":
        return [(start_position[0], start_position[1] + x) for x in range(1, distance+1)]
    elif direction == "D":
        return [(start_position[0], start_position[1] - x) for x in range(1, distance+1)]

def points_hit_by_wire(wire):
    points = []
    position = (0, 0)

    for instruction in wire:
        direction, distance = instruction[0], int(instruction[1:].strip())
        points += points_hit_single_instruction(position, direction, distance)
        position = points[-1]

    return set(points)

def intersection_points(wire1, wire2):
    return set.intersection(points_hit_by_wire(wire1), points_hit_by_wire(wire2))

def distance_to_closest_point(points):
    return min([abs(x[0]) + abs(x[1]) for x in points])

def str_to_wire(wire_str):
    return wire_str.split(",")

def closest_point_from_strs(wire1_str, wire2_str):
    return distance_to_closest_point(intersection_points(str_to_wire(wire1_str), str_to_wire(wire2_str)))

def read_wire_strs_from_file(filename):
    with open(filename, "r") as f:
        return [line for line in f.readlines()]


# test cases
assert closest_point_from_strs("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83") == 159
assert closest_point_from_strs("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7") == 135

WIRES = read_wire_strs_from_file("data/03.txt")
if len(WIRES) > 1:
    print(closest_point_from_strs(WIRES[0], WIRES[1])) #399
