"""
day 5: sunny with a chance of asteroids
https://adventofcode.com/2019/day/5
"""

from typing import List

import attr

@attr.s(slots=True)
class Parser:
    program = attr.ib(factory=list)
    replace_reads_value = attr.ib(default=None)
    return_rather_than_print = attr.ib(default=False)

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

    def _get_params(self, instruction_len):
        return self.program[self.index+1:self.index+instruction_len]

    def _get_params_with_modes(self, instruction_len, param_modes):
        return list(zip(self._get_params(instruction_len), param_modes))

    def _parse_instruction_opcode1(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        self.program[pms[2][0]] = self._get_value(*pms[0]) + self._get_value(*pms[1])
        self.index += instruction_len

    def _parse_instruction_opcode2(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        self.program[pms[2][0]] = self._get_value(*pms[0]) * self._get_value(*pms[1])
        self.index += instruction_len

    def _parse_instruction_opcode3(self, param_modes=list):
        instruction_len = 2
        ps = self._get_params(instruction_len)

        if self.replace_reads_value:
            value = self.replace_reads_value
        else:
            value = int(input("--> "))

        self.program[ps[0]] = value
        self.index += instruction_len

    def _parse_instruction_opcode4(self, param_modes=list):
        instruction_len = 2
        pms = self._get_params_with_modes(instruction_len, param_modes)

        value = self._get_value(*pms[0])
        self.index += instruction_len

        if self.return_rather_than_print:
            return value

        print(value)

    def _parse_instruction_opcode5(self, param_modes=list):
        instruction_len = 3
        pms = self._get_params_with_modes(instruction_len, param_modes)

        if self._get_value(*pms[0]):
            self.index = self._get_value(*pms[1])
        else:
            self.index += instruction_len

    def _parse_instruction_opcode6(self, param_modes=list):
        instruction_len = 3
        pms = self._get_params_with_modes(instruction_len, param_modes)

        if not self._get_value(*pms[0]):
            self.index = self._get_value(*pms[1])
        else:
            self.index += instruction_len

    def _parse_instruction_opcode7(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        if self._get_value(*pms[0]) < self._get_value(*pms[1]):
            self.program[pms[2][0]] = 1
        else:
            self.program[pms[2][0]] = 0

        self.index += instruction_len

    def _parse_instruction_opcode8(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        if self._get_value(*pms[0]) == self._get_value(*pms[1]):
            self.program[pms[2][0]] = 1
        else:
            self.program[pms[2][0]] = 0

        self.index += instruction_len

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
            if self.return_rather_than_print:
                return self._parse_instruction_opcode4(param_modes)
        elif opcode == 5:
            self._parse_instruction_opcode5(param_modes)
        elif opcode == 6:
            self._parse_instruction_opcode6(param_modes)
        elif opcode == 7:
            self._parse_instruction_opcode7(param_modes)
        elif opcode == 8:
            self._parse_instruction_opcode8(param_modes)
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
            val = self._parse_instruction()
            if val is not None and self.return_rather_than_print:
                return val

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

# stdout test cases
assert Parser([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], replace_reads_value=8, return_rather_than_print=True).parse() == 1
assert Parser([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], replace_reads_value=7, return_rather_than_print=True).parse() == 0
assert Parser([3, 3, 1108, -1, 8, 3, 4, 3, 99], replace_reads_value=8, return_rather_than_print=True).parse() == 1
assert Parser([3, 3, 1108, -1, 8, 3, 4, 3, 99], replace_reads_value=7, return_rather_than_print=True).parse() == 0
assert Parser([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], replace_reads_value=0, return_rather_than_print=True).parse() == 0
assert Parser([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], replace_reads_value=8, return_rather_than_print=True).parse() == 1
assert Parser([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], replace_reads_value=0, return_rather_than_print=True).parse() == 0
assert Parser([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], replace_reads_value=8, return_rather_than_print=True).parse() == 1

PROGRAM = Parser.init_from_file("data/05.txt")
PROGRAM.parse()
