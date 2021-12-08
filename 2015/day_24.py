""" day 24: it hangs in the balance """

from functools import reduce


def list_difference(minuend, *subtrahends):
    """safe list difference in case there are duplicate entries in minuend"""

    lst = minuend[:]
    for subtrahend in subtrahends:
        for elem in subtrahend:
            lst.remove(elem)

    return lst


def quantum_entanglement(group):
    return reduce(lambda x, y: x * y, group, 1)


def get_groups_for_weight(packages, weight):
    if len(packages) == 0:
        return

    for index in range(len(packages)):
        package_weight = packages[index]
        if package_weight > weight:
            continue

        if package_weight == weight:
            yield [package_weight]
            continue

        for subgroup in get_groups_for_weight(packages[index + 1 :], weight - package_weight):
            yield [package_weight] + subgroup


def supports_all_groups(packages, weight_per_group):
    if not packages:
        return True

    for group in get_groups_for_weight(packages, weight_per_group):
        if supports_all_groups(list_difference(packages, group), weight_per_group):
            return True

    return False


def potentially_better_than_best_group(group, best_group):
    if best_group is None:
        return True

    if len(group) < len(best_group):
        return True

    if len(group) == len(best_group) and quantum_entanglement(group) < quantum_entanglement(best_group):
        return True

    return False


def distribute_packages(packages, num_packages):
    packages.sort(reverse=True)

    weight_per_group = sum(packages) / num_packages
    best_group = None

    for group in get_groups_for_weight(packages, weight_per_group):
        if not potentially_better_than_best_group(group, best_group):
            continue

        if not supports_all_groups(list_difference(packages, group), weight_per_group):
            continue

        best_group = group

    print(best_group)

    return quantum_entanglement(best_group)


TEST_PACKAGES = list(range(1, 6)) + list(range(7, 12))
assert distribute_packages(TEST_PACKAGES, 3) == 99
assert distribute_packages(TEST_PACKAGES, 4) == 44

with open("data/24.txt") as f:
    PACKAGES = [int(x.strip()) for x in f.readlines() if x]

assert distribute_packages(PACKAGES, 3) == 11266889531
assert distribute_packages(PACKAGES, 4) == 77387711
