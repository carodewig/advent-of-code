""" ticket translation """

import re
from math import prod


def parse_input(input_str):
    reqs = {}
    req_pattern = re.compile("([A-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)")
    your_ticket = []
    nearby_tickets = []

    next_yours = False
    next_nearby = False
    for line in input_str.split("\n"):
        if not line:
            continue

        if req_match := re.match(req_pattern, line):
            req_name, lowleft, highleft, lowright, highright = req_match.groups()
            reqs[req_name] = [(int(lowleft), int(highleft)), (int(lowright), int(highright))]
            continue

        if line == "your ticket:":
            next_yours = True
            continue

        if line == "nearby tickets:":
            next_nearby = True
            next_yours = False
            continue

        ticket = [int(x) for x in line.split(",")]
        if next_yours:
            your_ticket = ticket
        elif next_nearby:
            nearby_tickets.append(ticket)

    return reqs, your_ticket, nearby_tickets


def matches_any_requirement(reqs, ticket_value):
    for req_list in reqs.values():
        for req_min, req_max in req_list:
            if req_min <= ticket_value <= req_max:
                return True

    return False


def invalid_values(reqs, _, nearby_tickets):
    error_rate = 0
    for ticket in nearby_tickets:
        matches_reqs = [(val, matches_any_requirement(reqs, val)) for val in ticket]
        for val, match in matches_reqs:
            if not match:
                error_rate += val
                break

    return error_rate


def valid_tickets(reqs, nearby_tickets):
    for ticket in nearby_tickets:
        matches_reqs = [(val, matches_any_requirement(reqs, val)) for val in ticket]
        for val, match in matches_reqs:
            if not match:
                break
        else:
            yield ticket


def prune_indexes(req_indexes):
    changed = None
    while changed is None or changed:
        changed = False
        for req in req_indexes:
            if len(req_indexes[req]) == 1:
                value = req_indexes[req][0]

                for sub_req in req_indexes:
                    if req != sub_req and value in req_indexes[sub_req]:
                        req_indexes[sub_req].remove(value)
                        changed = True


def part2_ticket_values(reqs, your_ticket, nearby_tickets):
    ticket_length = len(your_ticket)
    req_indexes = {req: list(range(ticket_length)) for req in reqs}

    for ticket in valid_tickets(reqs, nearby_tickets):
        for index, ticket_value in enumerate(ticket):
            for (req, req_list) in reqs.items():
                if index not in req_indexes[req]:
                    continue

                matches = False
                for req_min, req_max in req_list:
                    if req_min <= ticket_value <= req_max:
                        matches = True
                        break

                if not matches:
                    req_indexes[req].remove(index)

        prune_indexes(req_indexes)

    departure_indexes = [req_lst[0] for req, req_lst in req_indexes.items() if req.startswith("departure")]
    return prod([value for index, value in enumerate(your_ticket) if index in departure_indexes])


TEST_INPUT = """
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"""

assert invalid_values(*parse_input(TEST_INPUT)) == 71
part2_ticket_values(*parse_input(TEST_INPUT))

INPUT = ""
with open("data/16.txt") as fh:
    INPUT = fh.read()

assert invalid_values(*parse_input(INPUT)) == 25788
print(part2_ticket_values(*parse_input(INPUT)))
