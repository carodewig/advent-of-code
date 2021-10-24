from abc import ABC, abstractmethod

import attr


@attr.s
class Rule(ABC):
    @abstractmethod
    def possible_matches(self, example):
        """ possible match if the beginning of the string matches the rule """

    def exact_matches(self, example):
        for match in self.possible_matches(example):
            if match == example:
                yield match
                return


@attr.s
class StrRule(Rule):
    letter: str = attr.ib()

    def possible_matches(self, example):
        if example == "":
            return

        if example[0] == self.letter:
            yield self.letter


@attr.s
class ConcatRule(Rule):
    left_rule: Rule = attr.ib()
    right_rule: Rule = attr.ib()

    def possible_matches(self, example):
        if example == "":
            return

        for left_match in self.left_rule.possible_matches(example):
            for right_match in self.right_rule.possible_matches(example[len(left_match) :]):
                yield left_match + right_match


@attr.s
class OrRule(Rule):
    left_rule: Rule = attr.ib()
    right_rule: Rule = attr.ib()

    def possible_matches(self, example):
        yield from self.left_rule.possible_matches(example)
        yield from self.right_rule.possible_matches(example)


@attr.s
class DoublingRule(Rule):
    rule: Rule = attr.ib()

    def possible_matches(self, example):
        if example == "":
            return

        for candidate in self.rule.possible_matches(example):
            yield candidate

            for match in self.possible_matches(example[len(candidate) :]):
                yield candidate + match


@attr.s
class InsertionRule(Rule):
    left_rule: Rule = attr.ib()
    right_rule: Rule = attr.ib()

    def possible_matches(self, example):
        if example == "":
            return

        for lcand in self.left_rule.possible_matches(example):
            for substr_index in range(len(lcand), len(example)):
                for rcand in self.right_rule.possible_matches(example[substr_index:]):
                    if not example.endswith(rcand):
                        continue

                    if lcand + rcand == example:
                        yield lcand + rcand
                        continue

                    example_remaining = example[len(lcand) : len(example) - len(rcand)]
                    for match in self.possible_matches(example_remaining):
                        yield lcand + match + rcand


def get_top_level_rule(rule_strs):
    rules = {}

    def parse_for_index(index):
        if index in rules:
            return rules[index]

        rule_str = rule_strs[index].strip()

        # check for string rule first
        if '"' in rule_str:
            return StrRule(rule_str.replace('"', ""))

        # it has to be nested. work out which rules need to be predefined
        sub_rule_indices = {int(x) for sub in rule_str.split("|") for x in sub.strip().split(" ")}

        # go through and work out all those sub_rules
        for sub_index in sub_rule_indices:
            if sub_index == index:
                # skip self if it's a looping rule
                continue

            rules[sub_index] = parse_for_index(sub_index)

        # now actually instantiate our rule, go through a few cases
        # might have an OR rule
        return parse_str_to_rule(rule_str, index)

    def parse_str_to_rule(rule_str, index):
        rule_indices = [int(x) for sub in rule_str.split("|") for x in sub.strip().split(" ")]

        if index in rule_indices:
            if int(rule_indices[-1]) == index:
                return DoublingRule(rules[rule_indices[0]])

            return InsertionRule(rules[rule_indices[0]], rules[rule_indices[1]])

        if "|" in rule_str:
            rules_split = [x.strip() for x in rule_str.split("|")]
            return OrRule(
                parse_str_to_rule(rules_split[0], index), parse_str_to_rule(rules_split[1], index),
            )

        if len(rule_indices) == 1:
            return rules[rule_indices[0]]

        return ConcatRule(
            rules[rule_indices[0]],
            parse_str_to_rule(" ".join([str(x) for x in rule_indices[1:]]), index),
        )

    return parse_for_index(0)


def read_in_rules_and_examples(input_str):
    rules = {}
    examples = []

    for line in input_str.split("\n"):
        if ":" in line:
            index, rule = line.split(":")
            rules[int(index)] = rule.strip()

        elif line:
            examples.append(line)

    return rules, examples


def evaluate_rules_and_examples(input_str):
    rules_strs, examples = read_in_rules_and_examples(input_str)

    rule = get_top_level_rule(rules_strs)
    return sum([1 if next(rule.exact_matches(ex), None) else 0 for ex in examples])


TEST_STR = """
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"""


assert evaluate_rules_and_examples(TEST_STR) == 2

with open("data/test/19_part1.txt") as fh:
    assert evaluate_rules_and_examples(fh.read()) == 3


with open("data/test/19_part2.txt") as fh:
    assert evaluate_rules_and_examples(fh.read()) == 12


with open("data/19.txt") as fh:
    assert evaluate_rules_and_examples(fh.read()) == 102


with open("data/19_2.txt") as fh:
    assert evaluate_rules_and_examples(fh.read()) == 318
