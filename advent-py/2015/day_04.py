""" day 4: the ideal stocking stuffer """

import re
from hashlib import md5

PATTERN = re.compile("^0{5}.*")


def five_leading_zeros(hashed):
    return bool(re.match(PATTERN, hashed))


def process(key, pattern_to_match):
    index = 1
    while True:
        md5hash = md5(bytes(f"{key}{index}", encoding="utf8")).hexdigest()
        if bool(re.match(pattern_to_match, md5hash)):
            return index

        index += 1


assert process("abcdef", re.compile("^0{5}.*")) == 609043
assert process("pqrstuv", re.compile("^0{5}.*")) == 1048970

print(process("ckczppom", re.compile("^0{5}.*")))
print(process("ckczppom", re.compile("^0{6}.*")))
