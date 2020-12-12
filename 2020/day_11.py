
from itertools import product

from enum import Enum
from copy import deepcopy

class Seat(Enum):
    FLOOR = "."
    EMPTY = "L"
    OCCUPIED = "#"


def parse_seats(seat_str):
    return [[Seat(x) for x in line.strip()] for line in seat_str.split("\n") if line]

def count_occupied_seats(seats):
    return sum([sum([int(seat == Seat.OCCUPIED) for seat in row]) for row in seats])

def seat_range(seats, xs, ys):
    min_x, max_x = xs
    min_y, max_y = ys

    return [seats[y][min_x:max_x] for y in range(min_y, max_y)]

def step_options(x, y, max_x, max_y):
    return (max(0, x-1), min(max_x, x+2)), (max(0, y-1), min(max_y, y+2))

def occupied_seat_visible(seats, x, y, x_dir, y_dir):
    if x_dir == 0 and y_dir == 0:
        return False

    rows = len(seats)
    columns = len(seats[0])

    x += x_dir
    y += y_dir
    while 0 <= x < columns and 0 <= y < rows:
        if seats[y][x] == Seat.OCCUPIED:
            return True
        if seats[y][x] == Seat.EMPTY:
            return False

        x += x_dir
        y += y_dir

    return False

def step(seats, occupied_part1=True):
    new_seats = deepcopy(seats)

    rows = len(seats)
    columns = len(seats[0])

    for y in range(rows):
        for x in range(columns):
            seat_state = seats[y][x]
            if seat_state == Seat.FLOOR:
                continue

            if occupied_part1:
                occupied = count_occupied_seats(seat_range(seats, *step_options(x, y, columns, rows))) - int(seat_state == Seat.OCCUPIED)
            else:
                occupied = 0
                for (x_step, y_step) in product([-1, 0, 1], [-1, 0, 1]):
                    occupied += int(occupied_seat_visible(seats, x, y, x_step, y_step))

            if seat_state == Seat.EMPTY and occupied == 0:
                new_seats[y][x] = Seat.OCCUPIED
            elif seat_state == Seat.OCCUPIED and occupied >= 4 and occupied_part1:
                new_seats[y][x] = Seat.EMPTY
            elif seat_state == Seat.OCCUPIED and occupied >= 5 and not occupied_part1:
                new_seats[y][x] = Seat.EMPTY

    return new_seats


def run(seat_str, occupied_part1=True):
    # start greater than zero for the ability to loop nicely
    state = parse_seats(seat_str)
    changed = True
    idx = 0

    while changed is True:
        new_state = step(state, occupied_part1)
        changed = (state != new_state)

        state = new_state

    return count_occupied_seats(state)


def seats_to_str(seats):
    return "\n".join(["".join([x.value for x in row]) for row in seats])


TEST_SEATS = """
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"""

assert run(TEST_SEATS, occupied_part1=True) == 37
assert run(TEST_SEATS, occupied_part1=False) == 26

with open("data/11.txt") as fh:
    SEATS = fh.read()

print(run(SEATS, occupied_part1=True))
print(run(SEATS, occupied_part1=False))
