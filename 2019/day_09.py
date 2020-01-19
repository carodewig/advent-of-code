"""
day 9: sensor boost
https://adventofcode.com/2019/day/9
"""

from day_05 import Parser, test_parser

#Parser([109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]).parse()
assert len(str(Parser([1102,34915192,34915192,7,4,7,99,0], return_rather_than_print=True).parse())) == 16
assert Parser([104,1125899906842624,99], return_rather_than_print=True).parse() == 1125899906842624

BOOST = Parser.init_from_file("data/09.txt", replace_reads_value=[2])
BOOST.parse()
