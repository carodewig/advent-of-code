""" day 19: medicine for rudolph """

import queue
from copy import copy
import re
from collections import defaultdict


def invert_replacements(d):
    inverted = {}

    for a, listb in d.items():
        for b in listb:
            inverted[b] = a

    return inverted


def distinct_replaced_molecules(state, replacements):
    mol_pattern = re.compile(r"(e|[A-Z][a-z]*)")
    state_molecules = re.findall(mol_pattern, state)

    molecules = set()

    for index, molecule in enumerate(state_molecules):
        for rep_opt in replacements[molecule]:
            this_molecule = copy(state_molecules)
            this_molecule[index] = rep_opt
            molecules.add("".join(this_molecule))

    return len(molecules)


def steps_to_e(state, orig_replacements):
    replacements = invert_replacements(orig_replacements)
    prev_state = None

    steps = 0
    while state != "e":
        for from_str, to_str in replacements.items():
            if from_str in state:
                new_state = state.replace(from_str, to_str, 1)
                if "e" in new_state and new_state != "e":
                    # it's possible to hit "e" too early, avoid that
                    continue

                state = new_state
                steps += 1

        # no greedy solution possible
        if prev_state == state:
            return None

        prev_state = state

    return steps


def parse(replacement_str):
    pattern = re.compile(r"([A-z0-9]+) => ([A-z0-9]+)")
    replacements = defaultdict(list)

    for line in replacement_str.split("\n"):
        if match := re.match(pattern, line.strip()):
            in_mol, out_mol = match.groups()
            replacements[in_mol].append(out_mol)
        elif line:
            start_state = line.strip()

    return start_state, replacements


TEST_REPLACEMENT_STR1 = """
H => HO
H => OH
O => HH

HOH
"""


TEST_REPLACEMENT_STR2 = """
e => H
e => O
H => HO
H => OH
O => HH

HOH
"""


TEST_REPLACEMENT_STR3 = """
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
"""


assert distinct_replaced_molecules(*parse(TEST_REPLACEMENT_STR1)) == 4
assert steps_to_e(*parse(TEST_REPLACEMENT_STR2)) == 3
assert steps_to_e(*parse(TEST_REPLACEMENT_STR3)) == 6


with open("data/19.txt") as fh:
    REPLACEMENT_STR = fh.read()

print(distinct_replaced_molecules(*parse(REPLACEMENT_STR)))
print(steps_to_e(*parse(REPLACEMENT_STR)))
