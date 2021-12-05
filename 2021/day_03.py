""" day 3: binary diagnostic """


def negate_bit(bit):
    if bit == "1":
        return "0"
    elif bit == "0":
        return "1"

    return None


def get_most_common_bit(bits):
    zero_freq = bits.count("0")
    one_freq = bits.count("1")

    if zero_freq > one_freq:
        return "0"

    if zero_freq < one_freq:
        return "1"

    return None


def get_least_common_bit(bits):
    return negate_bit(get_most_common_bit(bits))


def power_consumption(report):
    numbers = [line.strip() for line in report if line]

    gamma = ""
    epsilon = ""

    for bits in zip(*numbers):
        most_common = get_most_common_bit(bits)
        gamma += most_common
        epsilon += negate_bit(most_common)

    return int(gamma, 2) * int(epsilon, 2)


def get_rating(report_imm, bit_freq_fn, default_bit=None):
    report = [line for line in report_imm if line]

    bits = ""

    while len(report) > 1:
        first_bits = [line[0] for line in report if line]
        remaining = [line[1:] for line in report if line]

        target_bit = bit_freq_fn(first_bits)
        if target_bit is None:
            target_bit = default_bit

        bits += target_bit
        report = [r for (f, r) in zip(first_bits, remaining) if target_bit == f and len(r) > 0]

    if report:
        bits += report[0]

    return int(bits, 2)


def life_support_rating(report):
    oxygen_rating = get_rating(report, get_most_common_bit, "1")
    co2_rating = get_rating(report, get_least_common_bit, "0")

    return oxygen_rating * co2_rating


def read_all_lines(filename):
    with open(filename) as f:
        return f.readlines()


REPORT_STR = """
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"""

assert power_consumption(REPORT_STR.strip().split("\n")) == 198
assert life_support_rating(REPORT_STR.strip().split("\n")) == 230

print(power_consumption(read_all_lines("data/day_03.txt")))
print(life_support_rating(read_all_lines("data/day_03.txt")))
