""" day 2: dive! """


def follow_course_part1(course_gen):
    pos_x = 0
    depth = 0

    for line in course_gen:
        if not line:
            continue

        direction, distance_str = line.strip().split(" ")
        distance = int(distance_str)
        if direction == "forward":
            pos_x += distance
        elif direction == "down":
            depth += distance
        elif direction == "up":
            depth -= distance

    return pos_x * depth


def follow_course_part2(course_gen):
    pos_x = 0
    depth = 0
    aim = 0

    for line in course_gen:
        if not line:
            continue

        direction, distance_str = line.strip().split(" ")
        distance = int(distance_str)
        if direction == "forward":
            pos_x += distance
            depth += aim * distance
        elif direction == "down":
            aim += distance
        elif direction == "up":
            aim -= distance

    return pos_x * depth


def read_course_from_file(filename):
    with open(filename) as f:
        for line in f:
            yield line


COURSE_STR = """
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""

assert follow_course_part1(COURSE_STR.split("\n")) == 150
assert follow_course_part2(COURSE_STR.split("\n")) == 900

print(follow_course_part1(read_course_from_file("data/day_02.txt")))
print(follow_course_part2(read_course_from_file("data/day_02.txt")))
