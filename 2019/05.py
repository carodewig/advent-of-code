"""
day 5: sunny with a chance of asteroids
https://adventofcode.com/2019/day/5
"""

from typing import List

import attr

@attr.s(slots=True)
class Parser:
    program = attr.ib(factory=list)
    index = attr.ib(init=False, default=0)
    backup_program = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.backup_program = list(self.program)

    def reset(self):
        self.program = list(self.backup_program)
        self.index = 0

    @classmethod
    def init_from_file_generator(cls, filename):
        with open(filename, "r") as file:
            for line in file.readlines():
                program_raw = [int(x) for x in line.strip().split(",")]
                yield Parser(program=program_raw)
    @classmethod
    def init_from_file(cls, filename):
        for parser in Parser.init_from_file_generator(filename):
            return parser

    def _get_value(self, parameter, mode):
        if mode == 1:
            return parameter
        return self.program[parameter]

    def _parse_instruction_opcode1(self, param_modes=list):
        instructions = self.program[self.index+1:self.index+4]
        self.program[instructions[2]] = self._get_value(instructions[0], param_modes[0]) + self._get_value(instructions[1], param_modes[1])
        self.index += 4

    def _parse_instruction_opcode2(self, param_modes=list):
        instructions = self.program[self.index+1:self.index+4]
        self.program[instructions[2]] = self._get_value(instructions[0], param_modes[0]) * self._get_value(instructions[1], param_modes[1])
        self.index += 4

    def _parse_instruction_opcode3(self, param_modes=list):
        position = self.program[self.index + 1]
        self.program[position] = int(input("--> "))
        self.index += 2

    def _parse_instruction_opcode4(self, param_modes=list):
        value = self._get_value(self.program[self.index + 1], param_modes[0])
        print(value)
        self.index += 2

    # exit by setting index outside range of program
    def _parse_instruction_opcode99(self):
        self.index = len(self.program)

    def _parse_instruction(self):
        opcode_with_params = f'{self.program[self.index]:05}'

        opcode = int(opcode_with_params[-2::])
        param_modes = [int(x) for x in opcode_with_params[:3][::-1]]

        if opcode == 1:
            self._parse_instruction_opcode1(param_modes)
        elif opcode == 2:
            self._parse_instruction_opcode2(param_modes)
        elif opcode == 3:
            self._parse_instruction_opcode3(param_modes)
        elif opcode == 4:
            self._parse_instruction_opcode4(param_modes)
        elif opcode == 99:
            self._parse_instruction_opcode99()
        else:
            print(f"Bad opcode {opcode}!")
            raise ValueError

    def parse(self, noun=None, verb=None):
        if noun is not None and verb is not None:
            self.program[1] = noun
            self.program[2] = verb

        while self.index < len(self.program):
            self._parse_instruction()

        return self.program[0]

    def parse_and_get_value_at_index(self, index):
        self.parse()
        return self.program[index]


# test cases
assert Parser([1, 0, 0, 0, 99]).parse() == 2
assert Parser([2, 3, 0, 3, 99]).parse() == 2
assert Parser([2, 4, 4, 5, 99, 0]).parse() == 2
assert Parser([1, 1, 1, 4, 99, 5, 6, 0, 99]).parse() == 30
assert Parser([1002, 4, 3, 4, 33]).parse_and_get_value_at_index(4) == 99
assert Parser([1101, 100, -1, 4, 0]).parse_and_get_value_at_index(4) == 99


PROGRAM = Parser.init_from_file("data/05.txt")
PROGRAM.parse()

