""" day 19: medicine for rudolph """

from copy import copy
import re
from collections import defaultdict

def invert_dict(d):
    inverted = defaultdict(list)

    for a, listb in d.items():
        for b in listb:
            inverted[b].append(a)

    return inverted

def len_iterator(it):
    return len(list(it))

def distinct_replaced_molecules(state, replacements):
    mol_pattern = re.compile(r"(e|[A-Z][a-z]*)")
    state_molecules = re.findall(mol_pattern, state)

    molecules = set()

    for index, molecule in enumerate(state_molecules):
        for rep_opt in replacements[molecule]:
            this_molecule = copy(state_molecules)
            this_molecule[index] = rep_opt
            molecules.add("".join(this_molecule))

    yield from molecules


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



assert len_iterator(distinct_replaced_molecules(*parse(TEST_REPLACEMENT_STR1))) == 4

with open("data/19.txt") as fh:
    REPLACEMENT_STR = fh.read()

print(len_iterator(distinct_replaced_molecules(*parse(REPLACEMENT_STR))))

