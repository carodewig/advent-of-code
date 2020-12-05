""" day 13: knights of the dinner table """

import re
from collections import defaultdict
from itertools import permutations
import math


def pair(*people):
    return tuple(sorted(people))


def parse_attendees(attendees):
    pattern = re.compile(r"([A-z]+) would ([A-z]+) ([0-9]+) happiness units by sitting next to ([A-z]+)\.")
    attendee_happiness = defaultdict(int)
    for attendee_str in attendees.split("\n"):
        if match := re.match(pattern, attendee_str):
            person1, gain_lose, units, person2 = match.groups()
            happiness = int(units) * (1 if gain_lose == "gain" else -1)

            attendee_happiness[pair(person1, person2)] += happiness

    return attendee_happiness


def total_happiness(attendee_sequence, attendee_happiness):
    happiness = 0
    for p in zip(attendee_sequence, attendee_sequence[1:] + attendee_sequence[:1]):
        happiness += attendee_happiness[pair(*p)]

    return happiness


def happiest(attendees, add_self=False):
    attendee_happiness = parse_attendees(attendees)
    attendee_list = list(set(person for pair in attendee_happiness.keys() for person in pair))

    if add_self:
        for attendee in attendee_list:
            attendee_happiness[pair("Me", attendee)] = 0
        attendee_list += "Me"

    # brute force bc kruskal's alg doesn't work :(
    highest_happiness = -math.inf
    for order in permutations(attendee_list):
        happiness = total_happiness(order, attendee_happiness)
        highest_happiness = max(highest_happiness, happiness)

    return highest_happiness


TEST_ATTENDEES = """
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
"""

assert happiest(TEST_ATTENDEES) == 330

with open("data/13.txt") as fh:
    print(happiest(fh.read(), add_self=True))
