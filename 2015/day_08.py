""" day 8: matchsticks """


import json


def code_length(string):
    return len(string)


def str_length(string):
    return len(eval(string))


def repr_length(string):
    return len(json.dumps(string))


def diff_chars_decoded(filename):
    char_diff = 0
    with open(filename) as fh:
        for _line in fh:
            line = _line.strip()
            char_diff += code_length(line) - str_length(line)

    return char_diff


def diff_chars_encoded(filename):
    char_diff = 0
    with open(filename) as fh:
        for _line in fh:
            line = _line.strip()
            char_diff += repr_length(line) - code_length(line)

    return char_diff


assert diff_chars_decoded("data/08-examples.txt") == 12
assert diff_chars_encoded("data/08-examples.txt") == 19

print(diff_chars_decoded("data/08.txt"))
print(diff_chars_encoded("data/08.txt"))
