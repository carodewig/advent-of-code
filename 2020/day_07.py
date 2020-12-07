""" day 7: handy haversacks """

import re

import attr


@attr.s
class Bag:
    color = attr.ib()
    contents = attr.ib(factory=list)

    def add(self, bag, number):
        self.contents.append((bag.color, int(number)))

    def holds(self, color):
        for (bag_color, _) in self.contents:
            if bag_color == color:
                return True

        return False


class Bags(dict):
    def get(self, color):
        if color not in self:
            self[color] = Bag(color)

        return self[color]

    def _bags_that_hold(self, color):
        for bag in self.values():
            if bag.holds(color):
                yield from self._bags_that_hold(bag.color)
                yield bag.color

    def number_of_bags_that_hold(self, color):
        return len(set(self._bags_that_hold(color)))

    def bags_within(self, color):
        return sum([number * (1 + self.bags_within(_color)) for (_color, number) in self[color].contents])


def parse_bags(bag_rules_str):
    bags = Bags(dict())
    bag_pattern = re.compile(r"(^|[0-9]+) *([A-z ]+) bags*")

    for bag_rule in bag_rules_str.split("\n"):
        matches = re.findall(bag_pattern, bag_rule)
        if not matches:
            continue

        bag = bags.get(matches[0][1])
        if len(matches) == 1:
            assert "no other" in bag_rule
            continue

        for (number, color) in matches[1:]:
            subbag = bags.get(color)
            bag.add(subbag, number)

    return bags


TEST_BAG_RULES_STR = """
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""

TEST_BAGS = parse_bags(TEST_BAG_RULES_STR)
assert TEST_BAGS.number_of_bags_that_hold("shiny gold") == 4
assert TEST_BAGS.bags_within("shiny gold") == 32

BAGS = None
with open("data/07.txt") as fh:
    BAGS = parse_bags(fh.read())

print(BAGS.number_of_bags_that_hold("shiny gold"))
print(BAGS.bags_within("shiny gold"))
