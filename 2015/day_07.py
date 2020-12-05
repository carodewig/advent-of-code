import re
from collections import namedtuple

import attr

Instruction = namedtuple("Instruction", ["op", "inputs", "output"])

OPS = {
    "AND": lambda x, y: int(x) & int(y),
    "OR": lambda x, y: int(x) | int(y),
    "LSHIFT": lambda x, y: int(x) << int(y),
    "RSHIFT": lambda x, y: int(x) >> int(y),
    "NOT": lambda x: ~int(x),
    "": int,
}


@attr.s
class Circuit:
    instruction_str = attr.ib()
    wires = attr.ib(init=False, factory=dict)

    def parse(self):
        def wire_or_value(val):
            try:
                return int(val)
            except ValueError:
                return val

        instructions = []
        for instruction in self.instruction_str.split("\n"):
            pattern = r"^([a-z0-9]+) ([A-Z]+) ([a-z0-9]+) -> ([a-z]+)$"
            if match := re.match(pattern, instruction):
                input1, operation, input2, output = match.groups()
                instructions.append(Instruction(operation, (wire_or_value(input1), wire_or_value(input2)), output,))
                continue

            pattern = r"^NOT ([a-z]+) -> ([a-z]+)$"
            if match := re.match(pattern, instruction):
                input1, output = match.groups()
                instructions.append(Instruction("NOT", (wire_or_value(input1),), output))
                continue

            pattern = r"^([a-z0-9]+) -> ([a-z]+)$"
            if match := re.match(pattern, instruction):
                input1, output = match.groups()
                instructions.append(Instruction("", (wire_or_value(input1),), output))
                continue

        return instructions

    def run(self, ignore_outputs=None):
        def ready(inputs):
            for in_val in inputs:
                if not (isinstance(in_val, int) or in_val in self.wires):
                    return False

            return True

        def get(name_or_value):
            value = self.wires.get(name_or_value, name_or_value)

            while value < 0:
                value += pow(2, 16)

            while value >= pow(2, 16):
                value -= pow(2, 16)

            return value

        instructions = self.parse()
        while len(instructions) > 0:
            for instruction in instructions:
                if ignore_outputs and instruction.output in ignore_outputs:
                    instructions.remove(instruction)
                    break

                if ready(instruction.inputs):
                    self.wires[instruction.output] = OPS[instruction.op](*[get(x) for x in instruction.inputs])
                    instructions.remove(instruction)
                    break

        self.wires = {x: get(x) for x in self.wires}


TEST_INSTRUCTIONS = """
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
"""

TEST_CIRCUIT = Circuit(TEST_INSTRUCTIONS)
TEST_CIRCUIT.run()
assert TEST_CIRCUIT.wires["i"] == 65079


with open("data/07.txt") as fh:
    REAL_INSTRUCTIONS = fh.read()

REAL_CIRCUIT = Circuit(REAL_INSTRUCTIONS)
REAL_CIRCUIT.run()
assert REAL_CIRCUIT.wires["a"] == 3176

REAL_CIRCUIT = Circuit(REAL_INSTRUCTIONS)
REAL_CIRCUIT.wires["b"] = 3176
REAL_CIRCUIT.run(ignore_outputs=["b"])
assert REAL_CIRCUIT.wires["a"] == 14710
