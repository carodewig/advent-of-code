""" day 10: adapter array """


def parse_adapters(adapter_str):
    adapter_list = [int(x.strip()) for x in adapter_str.split()]
    return sorted(adapter_list + [3 + max(adapter_list)])


def adapter_deltas(adapter_str):
    adapters = [0] + parse_adapters(adapter_str)
    deltas = [y - x for (x, y) in zip(adapters, adapters[1:])]
    return deltas


def part1_result(adapter_str):
    deltas = adapter_deltas(adapter_str)
    return deltas.count(1) * deltas.count(3)


def num_possible_chains(adapter_str):
    adapter_list = parse_adapters(adapter_str)

    cache = {}

    def _num_possible_chains(prev_adapter, remaining_adapters, max_adapter):
        if prev_adapter == max_adapter:
            return 1

        if prev_adapter in cache:
            return cache[prev_adapter]

        if not remaining_adapters:
            return 0

        total = 0
        for (index, adapter) in enumerate(remaining_adapters):
            diff = adapter - prev_adapter
            if 1 <= diff <= 3:
                num_for_adapter = _num_possible_chains(adapter, remaining_adapters[index + 1 :], max_adapter)
                cache[adapter] = num_for_adapter
                total += num_for_adapter

        return total

    return _num_possible_chains(0, adapter_list, max(adapter_list))


TEST_ADAPTERS1 = "16 10 15 5 1 11 7 19 6 12 4"
TEST_ADAPTERS2 = """
    28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3
"""


assert part1_result(TEST_ADAPTERS1) == 35
assert part1_result(TEST_ADAPTERS2) == 220

assert num_possible_chains(TEST_ADAPTERS1) == 8
assert num_possible_chains(TEST_ADAPTERS2) == 19208

with open("data/10.txt") as fh:
    ADAPTERS = fh.read()

print(part1_result(ADAPTERS))
print(num_possible_chains(ADAPTERS))
