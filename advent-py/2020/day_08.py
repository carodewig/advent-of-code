""" day 8: handheld halting """

import re
import pytest
import attr

from copy import copy


@attr.s
class Program:
    code = attr.ib()

    accumulator = attr.ib(default=0)
    index = attr.ib(default=0)

    operations = attr.ib(init=False)
    pattern = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.operations = {
            "acc": self.op_acc,
            "jmp": self.op_jmp,
            "nop": self.op_nop,
        }

        self.pattern = re.compile(r"^([A-z]+) ([\+\-0-9]+)$")

    def op_acc(self, value):
        self.accumulator += value
        self.index += 1

    def op_jmp(self, value):
        self.index += value

    def op_nop(self, _):
        self.index += 1

    def parse_instruction(self, line):
        return re.match(self.pattern, line.strip()).groups()

    def run(self):
        visited_indexes = [self.index]

        while len(visited_indexes) == len(set(visited_indexes)):
            # program terminated properly by moving to the line after the last instruction
            if self.index == len(self.code):
                return

            op, value = self.parse_instruction(self.code[self.index])
            self.operations[op](int(value))

            # at this point the index has already incremented
            # add to visited_indexes now so that loop will terminate if this is a repeat
            visited_indexes.append(self.index)

        raise Exception("Program failed to terminate")

    def reset(self):
        self.accumulator = 0
        self.index = 0

    @classmethod
    def from_str(cls, string):
        return cls([line.strip() for line in filter(None, string.split("\n"))])

    @classmethod
    def from_file(cls, filename):
        with open(filename) as fh:
            return cls.from_str(fh.read())


def uncorrupted_result(program):
    original_code = program.code

    for index in range(len(original_code)):
        program.code = copy(original_code)
        program.reset()

        if "jmp" in program.code[index]:
            program.code[index] = program.code[index].replace("jmp", "nop")
        elif "nop" in program.code[index]:
            program.code[index] = program.code[index].replace("nop", "jmp")

        if not exception_thrown(program.run):
            return program.accumulator


# basic implementation of pytest.raises
def exception_thrown(fn):
    try:
        fn()
    except Exception:
        return True

    return False


TEST_PROGRAM_STR = """
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"""

TEST_PROGRAM = Program.from_str(TEST_PROGRAM_STR)
assert exception_thrown(TEST_PROGRAM.run)
assert TEST_PROGRAM.accumulator == 5
assert uncorrupted_result(TEST_PROGRAM) == 8

PROGRAM = Program.from_file("data/08.txt")
assert exception_thrown(PROGRAM.run)
assert PROGRAM.accumulator == 1723
print(uncorrupted_result(PROGRAM))
