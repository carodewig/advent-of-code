"""
day 2: 1202 program alarm
https://adventofcode.com/2019/day/2
"""


def parse_program(program, noun=None, verb=None):
    index = 0
    instruction_length = 4

    if noun is not None and verb is not None:
        program[1] = noun
        program[2] = verb

    while index < len(program):
        opcode = program[index]
        try:
            input1, input2, output_position = program[index+1:index+instruction_length]
        except ValueError:
            break

        if opcode == 1:
            program[output_position] = program[input1] + program[input2]
        elif opcode == 2:
            program[output_position] = program[input1] * program[input2]
        elif opcode == 99:
            break
        else:
            print(f"Bad opcode {opcode}!")
            raise ValueError

        index += instruction_length

    return program[0]

def read_first_program_from_file(filename):
    with open(filename, "r") as f:
        for line in f.readlines():
            return [int(x) for x in line.strip().split(",")]

# test cases
assert parse_program([1, 0, 0, 0, 99]) == 2
assert parse_program([2, 3, 0, 3, 99]) == 2
assert parse_program([2, 4, 4, 5, 99, 0]) == 2
assert parse_program([1, 1, 1, 4, 99, 5, 6, 0, 99]) == 30

PROGRAM = read_first_program_from_file("data/02.txt")
for noun_opt in range(100):
    for verb_opt in range(100):
        if parse_program(list(PROGRAM), noun_opt, verb_opt) == 19690720:
            print(100*noun_opt + verb_opt)
