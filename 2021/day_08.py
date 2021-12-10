""" day 8: seven segment search """


def count_easy_digits(entry_gen):
    easy_digit_lengths = [2, 4, 3, 7]

    num_easy_digits = 0
    for (signal_patterns, output_digits) in entry_gen:
        for output_digit in output_digits:
            if len(output_digit) in easy_digit_lengths:
                num_easy_digits += 1

    return num_easy_digits


def determine_translations(lst):
    """
    I tried to do this generically with building up interim translations
    to "official" digit representations, i.e. 1 == cf. unfortunately there
    didn't seem to be enough information available to do that broadly...
    so picking out features of each number in relation to others ended up
    being much more successful
    """

    def overlaps(s1, s2):
        # all elements of s1 are in s2
        for char in s1:
            if char not in s2:
                return False
        return True

    res = [None for _ in range(10)]

    # one only has 2 strikes
    res[1] = [x for x in lst if len(x) == 2][0]
    res[4] = [x for x in lst if len(x) == 4][0]
    res[7] = [x for x in lst if len(x) == 3][0]
    res[8] = [x for x in lst if len(x) == 7][0]

    options_for_069 = [x for x in lst if len(x) == 6]
    options_for_235 = [x for x in lst if len(x) == 5]

    # 3 is the only one of 2,3,5 that overlaps 1
    res[3] = [x for x in options_for_235 if overlaps(res[1], x)][0]

    # 9 is the only one of 0,6,9 that overlaps 3
    res[9] = [x for x in options_for_069 if overlaps(res[3], x)][0]

    # between 0 and 6, only 0 overlaps 7
    res[0] = [x for x in options_for_069 if x != res[9] and overlaps(res[7], x)][0]

    # 6 is just what's left of 0,6,9
    res[6] = [x for x in options_for_069 if x != res[0] and x != res[9]][0]

    # we can use 6 and 8 to isolate the top right bar in the digit repr
    # that bar will be present in 2 but not 5
    top_right_bar_char = list(set(res[8]).difference(set(res[6])))[0]
    res[2] = [x for x in options_for_235 if x != res[3] and top_right_bar_char in x][0]

    # 5 is just what's left of 2,3,5
    res[5] = [x for x in options_for_235 if x != res[2] and x != res[3]][0]

    return res


def sum_output_digits(entry_gen):
    def output_digits():
        for entry in entry_gen:
            translations = determine_translations(entry[0])
            yield int("".join([str(translations.index(digit)) for digit in entry[1]]))

    return sum(output_digits())


def parse_entries(entry_str):
    def order_digits(e):
        return "".join(sorted(e[:]))

    def parse_entry(l):
        signal_patterns = [order_digits(x) for x in l.split("|")[0].strip().split(" ")]
        output_digits = [order_digits(x) for x in l.split("|")[1].strip().split(" ")]

        return signal_patterns, output_digits

    for line in entry_str.split("\n"):
        if not line:
            continue

        yield parse_entry(line)


TEST_ENTRY = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf"
TEST_ENTRIES = """
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce
"""

assert count_easy_digits(parse_entries(TEST_ENTRY)) == 0
assert count_easy_digits(parse_entries(TEST_ENTRIES)) == 26


with open("data/day_08.txt") as f:
    ENTRIES = f.read()

assert count_easy_digits(parse_entries(ENTRIES)) == 367


assert sum_output_digits(parse_entries(TEST_ENTRY)) == 5353
assert sum_output_digits(parse_entries(TEST_ENTRIES)) == 61229

assert sum_output_digits(parse_entries(ENTRIES)) == 974512
