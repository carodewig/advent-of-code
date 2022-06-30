""" day 14: reindeer olympics """

import re

from collections import defaultdict
import attr


@attr.s
class Reindeer:
    name = attr.ib()
    speed = attr.ib()
    fly_duration = attr.ib()
    rest_duration = attr.ib()

    distance = attr.ib(init=False, default=0)

    seconds_flown = attr.ib(init=False, default=0)
    seconds_rested = attr.ib(init=False, default=0)

    def fly(self):
        distance = 0
        while True:
            for _ in range(self.fly_duration):
                distance += self.speed
                yield (self.name, distance)

            for _ in range(self.rest_duration):
                yield (self.name, distance)


PATTERN = r"([A-z]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds."

stable = []
with open("data/14.txt") as fh:
    for line in fh:
        if match := re.match(PATTERN, line):
            name, speed, fly, rest = match.groups()
            stable.append(Reindeer(name, int(speed), int(fly), int(rest)))


scores = defaultdict(int)
for seconds, distances in enumerate(zip(*[reindeer.fly() for reindeer in stable]), 1):
    max_distance = max([x[1] for x in distances])

    for (name, distance) in distances:
        if distance == max_distance:
            scores[name] += 1

    if seconds == 2503:
        print(max(scores.values()))
        break
