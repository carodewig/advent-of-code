"""
day 9: sensor boost
https://adventofcode.com/2019/day/9
"""

from day_05 import IntcodeComputer, test_intcode_computer

test_intcode_computer()

# actual program -- convert into test cases for validation
BOOST = IntcodeComputer.init_from_file("data/09.txt", replace_stdin=[1])
assert BOOST.parse_and_get_last_value() == 3013554615

BOOST = IntcodeComputer.init_from_file("data/09.txt", replace_stdin=[2])
assert BOOST.parse_and_get_last_value() == 50158
