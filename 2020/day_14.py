""" day 14: docking data """

import re

# I don't have internet and I'm sure python has built-in binary processing
# but I don't know how it works
# so just implement from scratch
def int_to_binary(val, bits=36):
    binary_str = ["0" for _ in range(bits)]
    for (index, bit_expr) in enumerate(reversed(range(bits))):
        if 2 ** bit_expr <= val:
            binary_str[index] = "1"
            val -= 2 ** bit_expr

        if val == 0:
            break

    return "".join(binary_str)


def binary_to_int(bitstr):
    return sum([2 ** index for (index, val) in enumerate(reversed(bitstr)) if val == "1"])


def apply_mask_v1(mask, intval):
    bitstr = int_to_binary(intval)
    masked_str = "".join([maskval if maskval != "X" else bitval for (bitval, maskval) in zip(bitstr, mask)])
    return binary_to_int(masked_str)


def all_possibilities(masked_str):
    if "X" not in masked_str:
        yield masked_str
        return

    yield from all_possibilities(masked_str.replace("X", "0", 1))
    yield from all_possibilities(masked_str.replace("X", "1", 1))


def apply_mask_v2(mask, intval):
    bitstr = int_to_binary(intval)
    masked_str = "".join([maskval if maskval != "0" else bitval for (bitval, maskval) in zip(bitstr, mask)])
    for poss in all_possibilities(masked_str):
        yield binary_to_int(poss)


def parse_program(program_str):
    mask_pattern = re.compile(r"mask = ([X01]{36})")
    write_pattern = re.compile(r"mem\[([0-9]+)\] = ([0-9]+)")

    for line_index, line in enumerate(program_str.split("\n")):
        print(f"working on line {str(line_index+1)}: {line}")

        if match := re.match(mask_pattern, line.strip()):
            mask_str = match.groups()[0]
            yield (mask_str,)

        if match := re.match(write_pattern, line.strip()):
            index, value = match.groups()
            yield (int(index), int(value))


def run_program_v1(program_str):
    mask = ""
    mem = {}

    for instruction in parse_program(program_str):
        if len(instruction) == 1:
            mask = instruction[0]

        else:
            index, value = instruction
            mem[index] = apply_mask_v1(mask, value)

    return sum(mem.values())


def run_program_v2(program_str):
    mask = ""
    mem = {}

    for instruction in parse_program(program_str):
        if len(instruction) == 1:
            mask = instruction[0]

        else:
            index, value = instruction
            for masked_index in apply_mask_v2(mask, index):
                mem[masked_index] = value

    return sum(mem.values())


assert int_to_binary(11) == "000000000000000000000000000000001011"
assert int_to_binary(64) == "000000000000000000000000000001000000"
assert int_to_binary(73) == "000000000000000000000000000001001001"
assert int_to_binary(101) == "000000000000000000000000000001100101"

assert binary_to_int(int_to_binary(11)) == 11
assert binary_to_int(int_to_binary(64)) == 64
assert binary_to_int(int_to_binary(73)) == 73
assert binary_to_int(int_to_binary(101)) == 101


TEST_INITIALIZATION_PROGRAM = """
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"""
assert run_program_v1(TEST_INITIALIZATION_PROGRAM) == 165

TEST_INITIALIZATION_PROGRAM = """
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"""
assert run_program_v2(TEST_INITIALIZATION_PROGRAM) == 208


INITIALIZATION_PROGRAM = ""
with open("data/14.txt") as fh:
    INITIALIZATION_PROGRAM = fh.read()


assert run_program_v1(INITIALIZATION_PROGRAM) == 17028179706934
print(run_program_v2(INITIALIZATION_PROGRAM))
