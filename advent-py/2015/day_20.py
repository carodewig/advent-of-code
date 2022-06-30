""" infinite elves and infinite houses """

import math
import itertools

from functools import reduce


def get_value(primes, exps):
    return int(reduce(lambda x, y: x * y, [math.pow(p, ex) for (p, ex) in zip(primes, exps)], 1))


# at first I went for a brute force approach but that was untenable for part 2
# I saw an idea on the reddit thread to generate numbers with large numbers of prime
# factors and just test those
# I don't love the "human heuristic" aspect of setting manual upper bounds on
# exponents, but it works well here
def house_number(num_presents, elf_house_limit, presents_delivered_factor):
    lowest_house = math.inf

    # number needs to be attainable... set some upper bounds on exponent values
    primes = [2, 3, 5, 7, 11, 13, 17]
    exponents = [15, 10, 5, 5, 5, 5, 5]

    for prime_exp_candidates in itertools.product(*[range(exp) for exp in exponents]):
        house_candidate = get_value(primes, prime_exp_candidates)
        presents = 0

        if house_candidate >= lowest_house:
            # we've already found a better house than this so skip it
            continue

        # go through all factors of house candidate by iterating through possible prime exps
        for factor_exps in itertools.product(*[range(exp + 1) for exp in prime_exp_candidates]):
            factor = get_value(primes, factor_exps)

            # elves might only visit the first X houses
            if house_candidate / factor <= elf_house_limit:
                presents += factor * presents_delivered_factor

        if presents >= num_presents and house_candidate < lowest_house:
            lowest_house = house_candidate

    return lowest_house


print(house_number(36000000, math.inf, 10))
print(house_number(36000000, 50, 11))
