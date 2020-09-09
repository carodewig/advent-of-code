"""
day 13: care package
"""

import attr

from collections import defaultdict
from enum import Enum
from math import ceil, floor


@attr.s
class Reaction:
    inputs = attr.ib()
    outputs = attr.ib()

    def produces(self, element):
        return self.outputs[element]

    def __str__(self):
        return (
            " + ".join([f"{amount} {element}" for (element, amount) in self.inputs.items() if amount > 0])
            + " => "
            + " + ".join([f"{amount} {element}" for (element, amount) in self.outputs.items() if amount > 0])
        )

    @classmethod
    def init_from_str(cls, reaction_str):
        inputs_str, outputs_str = reaction_str.split("=>")

        def parse_str_to_dict(components_str):
            elements = defaultdict(int)
            for component in components_str.split(", "):
                amount, element = [x.strip() for x in component.split()]
                elements[element] = int(amount)

            return elements

        return cls(parse_str_to_dict(inputs_str), parse_str_to_dict(outputs_str))


@attr.s
class Nanofactory:
    reactions = attr.ib()
    leftovers = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.leftovers = defaultdict(int)

    def find_reaction_that_makes(self, element):
        for reaction in self.reactions:
            if reaction.produces(element):
                return reaction

    # work out how much ore is needed to make any element
    # greedy algorithm works here because "every chemical is produced by exactly one reaction"
    def _ore_for(self, amount, element):
        if amount == 0:
            return 0

        if element == "ORE":
            return amount

        # first see if we can use what we already made
        if element in self.leftovers:
            # if there's already plenty, just use that
            if self.leftovers[element] >= amount:
                self.leftovers[element] -= amount
                return 0

            # otherwise, use up what we had left and produce the rest
            amount_to_make = amount - self.leftovers[element]
            self.leftovers[element] = 0

        else:
            amount_to_make = amount

        # work out which reaction to use and how much to make
        reaction = self.find_reaction_that_makes(element)
        amount_per_reaction = reaction.produces(element)
        number_of_passes = int(ceil(amount_to_make / amount_per_reaction))

        # make sure to note anything that's leftover
        self.leftovers[element] += amount_per_reaction * number_of_passes - amount_to_make

        # determine ore required for my inputs
        baby_ores = [self._ore_for(number_of_passes * amt, elem) for (elem, amt) in reaction.inputs.items()]
        return sum(baby_ores)

    def ore_for(self, amount, element):
        # start a new reaction by emptying leftovers dict
        self.leftovers.clear()
        return self._ore_for(amount, element)

    def fuel_from_ore(self, amount):
        # first figure out the max ore required for 1 fuel
        ore_for_one_fuel = self.ore_for(1, "FUEL")
        total_fuel = 1
        amount -= ore_for_one_fuel

        # now ballpark how many more fuels you could make and make them
        while amount > ore_for_one_fuel:
            min_addtl_fuel = int(floor(amount / ore_for_one_fuel))
            ore = self._ore_for(min_addtl_fuel, "FUEL")
            amount -= ore
            total_fuel += min_addtl_fuel

        # but there may be enough leftover to make fuel for cheap, try that
        while amount >= 0:
            ore = self._ore_for(1, "FUEL")
            amount -= ore
            if amount < 0:
                break
            total_fuel += 1

        # ore exhausted return total
        return total_fuel

    @classmethod
    def init_from_list(cls, reactions_list):
        reactions = []
        for reaction_str in reactions_list:
            if not reaction_str.strip():
                continue
            reactions.append(Reaction.init_from_str(reaction_str))

        return cls(reactions)

    @classmethod
    def init_from_file(cls, filename):
        with open(filename) as f:
            # use readlines since files will be small
            return cls.init_from_list(f.readlines())


assert 31 == Nanofactory.init_from_file("data/14_tests/1.txt").ore_for(1, "FUEL")
assert 165 == Nanofactory.init_from_file("data/14_tests/2.txt").ore_for(1, "FUEL")
assert 13312 == Nanofactory.init_from_file("data/14_tests/3.txt").ore_for(1, "FUEL")
assert 180697 == Nanofactory.init_from_file("data/14_tests/4.txt").ore_for(1, "FUEL")
assert 2210736 == Nanofactory.init_from_file("data/14_tests/5.txt").ore_for(1, "FUEL")

assert 82892753 == Nanofactory.init_from_file("data/14_tests/3.txt").fuel_from_ore(10 ** 12)
assert 5586022 == Nanofactory.init_from_file("data/14_tests/4.txt").fuel_from_ore(10 ** 12)
assert 460664 == Nanofactory.init_from_file("data/14_tests/5.txt").fuel_from_ore(10 ** 12)

NANOFACTORY = Nanofactory.init_from_file("data/14.txt")
print(NANOFACTORY.ore_for(1, "FUEL"))
print(NANOFACTORY.fuel_from_ore(10 ** 12))
