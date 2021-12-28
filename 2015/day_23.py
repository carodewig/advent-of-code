""" day 23: opening the turing lock """


class Computer:
    def __init__(self):
        self.registers = {"a": 0, "b": 0}
        self.instruction_index = 0

        self.instruction_fns = {
            "hlf": self.inst_hlf,
            "tpl": self.inst_tpl,
            "inc": self.inst_inc,
            "jmp": self.inst_jmp,
            "jie": self.inst_jie,
            "jio": self.inst_jio,
        }

    def inst_hlf(self, r):
        self.registers[r] /= 2
        self.instruction_index += 1

    def inst_tpl(self, r):
        self.registers[r] *= 3
        self.instruction_index += 1

    def inst_inc(self, r):
        self.registers[r] += 1
        self.instruction_index += 1

    def inst_jmp(self, offset):
        self.instruction_index += int(offset)

    def inst_jie(self, r, offset):
        if self.registers[r] % 2 == 0:
            self.instruction_index += int(offset)
        else:
            self.instruction_index += 1

    def inst_jio(self, r, offset):
        if self.registers[r] == 1:
            self.instruction_index += int(offset)
        else:
            self.instruction_index += 1

    def run_program(self, program_str):
        program = program_str.strip().split("\n")
        while 0 <= self.instruction_index < len(program):
            inst, *args = program[self.instruction_index].replace(",", "").split(" ")
            self.instruction_fns[inst](*args)

        return self.registers["b"]


with open("data/23.txt") as f:
    PROGRAM = f.read()

c = Computer()
c.run_program(PROGRAM)
assert c.registers["b"] == 170

c = Computer()
c.registers["a"] = 1
c.run_program(PROGRAM)
assert c.registers["b"] == 247
