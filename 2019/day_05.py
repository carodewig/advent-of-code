"""
day 5: sunny with a chance of asteroids
https://adventofcode.com/2019/day/5
"""

from typing import List

import attr


@attr.s(slots=True)
class IntcodeComputer:
    program = attr.ib(factory=list)

    replace_stdin = attr.ib(factory=list)
    replace_stdout = attr.ib(default=False)

    index = attr.ib(init=False, default=0)
    relative_base = attr.ib(init=False, default=0)
    alive = attr.ib(init=False, default=True)

    backup_program = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.program = list(self.program)
        self.backup_program = list(self.program)

    def is_alive(self):
        return self.alive

    def reset(self):
        self.program = list(self.backup_program)
        self.index = 0
        self.relative_base = 0
        self.alive = True
        self.replace_stdin = list()

    @classmethod
    def init_from_file_generator(cls, filename, **kwargs):
        with open(filename, "r") as file:
            for line in file.readlines():
                program_raw = [int(x) for x in line.strip().split(",")]
                yield IntcodeComputer(program=program_raw, **kwargs)

    @classmethod
    def init_from_file(cls, filename, **kwargs):
        for intcode_computer in IntcodeComputer.init_from_file_generator(filename, **kwargs):
            return intcode_computer

    def _get_value_program(self, index):
        if index < len(self.program):
            return self.program[index]

        return 0

    def _get_value(self, parameter, mode):
        if mode == 0:
            return self._get_value_program(parameter)

        if mode == 2:
            return self._get_value_program(self.relative_base + parameter)

        return parameter

    def _get_value_write(self, parameter, mode):
        if mode == 2:
            return self.relative_base + parameter

        return parameter

    def _set_value(self, index, value):
        if index < len(self.program):
            self.program[index] = value
        else:
            self.program.extend([0 for _ in range(10 + index - len(self.program))])
            self.program[index] = value

    def _get_params(self, instruction_len):
        if self.index + instruction_len >= len(self.program):
            self.program.extend([0 for _ in range(10 + self.index + instruction_len - len(self.program))])

        return self.program[self.index + 1 : self.index + instruction_len]

    def _get_params_with_modes(self, instruction_len, param_modes):
        return list(zip(self._get_params(instruction_len), param_modes))

    def _parse_instruction_opcode1(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        self._set_value(self._get_value_write(*pms[2]), self._get_value(*pms[0]) + self._get_value(*pms[1]))
        self.index += instruction_len

    def _parse_instruction_opcode2(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        self._set_value(self._get_value_write(*pms[2]), self._get_value(*pms[0]) * self._get_value(*pms[1]))
        self.index += instruction_len

    def _parse_instruction_opcode3(self, param_modes=list):
        instruction_len = 2
        pms = self._get_params_with_modes(instruction_len, param_modes)
        ps = self._get_params(instruction_len)

        if self.replace_stdin:
            value = self.replace_stdin.pop(0)
        else:
            value = int(input("--> "))

        if pms[0][1] == 2:
            self._set_value(ps[0] + self.relative_base, value)
        else:
            self._set_value(ps[0], value)

        self.index += instruction_len

    def _parse_instruction_opcode4(self, param_modes=list):
        instruction_len = 2
        pms = self._get_params_with_modes(instruction_len, param_modes)

        value = self._get_value(*pms[0])
        self.index += instruction_len

        if self.replace_stdout:
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
            self._set_value(self._get_value_write(*pms[2]), 1)
        else:
            self._set_value(self._get_value_write(*pms[2]), 0)

        self.index += instruction_len

    def _parse_instruction_opcode8(self, param_modes=list):
        instruction_len = 4
        pms = self._get_params_with_modes(instruction_len, param_modes)

        if self._get_value(*pms[0]) == self._get_value(*pms[1]):
            self._set_value(self._get_value_write(*pms[2]), 1)
        else:
            self._set_value(self._get_value_write(*pms[2]), 0)

        self.index += instruction_len

    def _parse_instruction_opcode9(self, param_modes=list):
        instruction_len = 2
        pms = self._get_params_with_modes(instruction_len, param_modes)
        self.relative_base += self._get_value(*pms[0])

        self.index += instruction_len

    # exit by setting index outside range of program
    def _parse_instruction_opcode99(self):
        self.alive = False

    def _parse_instruction(self):
        opcode_with_params = f"{self.program[self.index]:05}"

        opcode = int(opcode_with_params[-2::])
        param_modes = [int(x) for x in opcode_with_params[:3][::-1]]

        if opcode == 1:
            self._parse_instruction_opcode1(param_modes)
        elif opcode == 2:
            self._parse_instruction_opcode2(param_modes)
        elif opcode == 3:
            self._parse_instruction_opcode3(param_modes)
        elif opcode == 4:
            val = self._parse_instruction_opcode4(param_modes)
            if self.replace_stdout:
                return val
        elif opcode == 5:
            self._parse_instruction_opcode5(param_modes)
        elif opcode == 6:
            self._parse_instruction_opcode6(param_modes)
        elif opcode == 7:
            self._parse_instruction_opcode7(param_modes)
        elif opcode == 8:
            self._parse_instruction_opcode8(param_modes)
        elif opcode == 9:
            self._parse_instruction_opcode9(param_modes)
        elif opcode == 99:
            self._parse_instruction_opcode99()
        else:
            print(f"Bad opcode {opcode}!")
            raise ValueError

    def parse(self, noun=None, verb=None, stop_on_yield=False):
        if noun is not None and verb is not None:
            self.program[1] = noun
            self.program[2] = verb

        while self.alive:
            val = self._parse_instruction()
            if val is not None and self.replace_stdout:
                yield val

                if stop_on_yield:
                    return

    def parse_and_get_value_at_index(self, index):
        for _ in self.parse():
            continue

        return self.program[index]

    def parse_and_get_first_value(self):
        self.replace_stdout = True
        for val in self.parse(stop_on_yield=True):
            return val

    def parse_and_get_last_value(self):
        self.replace_stdout = True

        last_val = None
        for val in self.parse():
            last_val = val

        return last_val

    def run(self):
        for _ in self.parse():
            continue


def test_intcode_computer():
    # test cases
    assert IntcodeComputer([1, 0, 0, 0, 99]).parse_and_get_value_at_index(0) == 2
    assert IntcodeComputer([2, 3, 0, 3, 99]).parse_and_get_value_at_index(0) == 2
    assert IntcodeComputer([2, 4, 4, 5, 99, 0]).parse_and_get_value_at_index(0) == 2
    assert IntcodeComputer([1, 1, 1, 4, 99, 5, 6, 0, 99]).parse_and_get_value_at_index(0) == 30
    assert IntcodeComputer([1002, 4, 3, 4, 33]).parse_and_get_value_at_index(4) == 99
    assert IntcodeComputer([1101, 100, -1, 4, 0]).parse_and_get_value_at_index(4) == 99

    # "stdout" test cases
    test_program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]
    assert IntcodeComputer(test_program, replace_stdin=[8]).parse_and_get_first_value() == 1
    assert IntcodeComputer(test_program, replace_stdin=[7]).parse_and_get_first_value() == 0

    test_program = [3, 3, 1108, -1, 8, 3, 4, 3, 99]
    assert IntcodeComputer(test_program, replace_stdin=[8]).parse_and_get_first_value() == 1
    assert IntcodeComputer(test_program, replace_stdin=[7]).parse_and_get_first_value() == 0

    test_program = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
    assert IntcodeComputer(test_program, replace_stdin=[0]).parse_and_get_first_value() == 0
    assert IntcodeComputer(test_program, replace_stdin=[8]).parse_and_get_first_value() == 1

    test_program = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
    assert IntcodeComputer(test_program, replace_stdin=[0]).parse_and_get_first_value() == 0
    assert IntcodeComputer(test_program, replace_stdin=[8]).parse_and_get_first_value() == 1

    assert len(str(IntcodeComputer([1102, 34915192, 34915192, 7, 4, 7, 99, 0]).parse_and_get_first_value())) == 16
    assert IntcodeComputer([104, 1125899906842624, 99]).parse_and_get_first_value() == 1125899906842624


if __name__ == "__main__":
    test_intcode_computer()

    PROGRAM = IntcodeComputer.init_from_file("data/05.txt")
    print(PROGRAM.parse_and_get_last_value())
