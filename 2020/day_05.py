""" day 5: binary boarding """


def parse_row(assignment):
    return int(assignment[:7].replace("F", "0").replace("B", "1"), 2)


def parse_seat(assignment):
    return int(assignment[-3:].replace("L", "0").replace("R", "1"), 2)


def seat_id(assignment):
    return parse_row(assignment) * 8 + parse_seat(assignment)


def find_open_seat(seat_ids):
    for seat in range(seat_ids[1], seat_ids[-1]):
        if seat not in seat_ids:
            if seat - 1 in seat_ids and seat + 1 in seat_ids:
                return seat


assert seat_id("FBFBBFFRLR") == 357
assert seat_id("BFFFBBFRRR") == 567
assert seat_id("FFFBBBFRRR") == 119
assert seat_id("BBFFBBFRLL") == 820


SEAT_IDS = []
with open("data/05.txt") as fh:
    SEAT_IDS = sorted([seat_id(line.strip()) for line in fh.readlines()])

MAX_SEAT_ID = SEAT_IDS[-1]
assert MAX_SEAT_ID == 890
assert find_open_seat(SEAT_IDS) == 651
