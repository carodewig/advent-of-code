"""
day 12: n-body problem
"""

from collections import defaultdict, namedtuple
from copy import deepcopy
from functools import reduce
from itertools import combinations
from math import gcd


import attr

Vector = namedtuple("Vector", "x y z")


def add_vectors(v1, v2):
    return Vector(*[x1 + x2 for x1, x2 in zip(v1, v2)])


def mult_vector(s, v):
    return Vector(*[s * x for x in v])


def sub_vectors(v1, v2):
    return add_vectors(v1, mult_vector(-1, v2))


def norm_delta(v1, v2):
    return Vector(*[int((y - x) / abs(y - x)) if x != y else 0 for x, y in zip(v1, v2)])


# get least common multiplier from list of integers
def lcm(nums):
    def _lcm(num1, num2):
        return int((num1 * num2) / gcd(num1, num2))

    return reduce(_lcm, nums)


@attr.s(slots=True)
class Moon:
    position = attr.ib()
    velocity = attr.ib(default=Vector(0, 0, 0))

    def kinetic_energy(self):
        return sum([abs(x) for x in self.velocity])

    def potential_energy(self):
        return sum([abs(x) for x in self.position])

    def total_energy(self):
        return self.kinetic_energy() * self.potential_energy()

    def to_str(self):
        return f"pos={self.position}, vel={self.velocity}"


@attr.s(slots=True)
class System:
    moons = attr.ib()
    num_steps = attr.ib(init=False, default=0)
    prior_states = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.prior_states = defaultdict(list)

    def moon_pairs(self):
        return combinations(self.moons, 2)

    def apply_gravity(self):
        for (moon1, moon2) in self.moon_pairs():
            delta_v = norm_delta(moon1.position, moon2.position)
            moon1.velocity = add_vectors(moon1.velocity, delta_v)
            moon2.velocity = sub_vectors(moon2.velocity, delta_v)

    def apply_velocity(self):
        for moon in self.moons:
            moon.position = add_vectors(moon.position, moon.velocity)

    def step(self, n=1):
        for _ in range(n):
            self.apply_gravity()
            self.apply_velocity()
            self.num_steps += 1

    def cycle_check(self, axis, steps_per_check=1):
        history_along_axis = self.prior_states[axis]
        history_length = len(history_along_axis)

        if history_length == len(set(history_along_axis)):
            return None

        for check_step in range(history_length - steps_per_check - 1, history_length):
            check_state = history_along_axis[check_step]
            for step in range(check_step):
                if check_state == history_along_axis[step]:
                    return check_step - step

        return None

    def track_state(self):
        for axis in range(3):
            state = tuple([moon.position[axis] for moon in self.moons] + [moon.velocity[axis] for moon in self.moons])
            self.prior_states[axis].append(state)

    def step_and_check_for_cycles(self):
        cycle_period_by_axis = [None, None, None]
        self.track_state()

        step_size = 1000

        while True:
            for _ in range(step_size):
                self.step(1)
                self.track_state()

            for axis in range(3):
                possible_cycle = self.cycle_check(axis, step_size)
                if possible_cycle and not cycle_period_by_axis[axis]:
                    cycle_period_by_axis[axis] = possible_cycle

            if all([x is not None for x in cycle_period_by_axis]):
                print(cycle_period_by_axis)
                return lcm(cycle_period_by_axis)

            if self.num_steps % 1000 == 0:
                print(f"Still running, on step {self.num_steps}. Current cycles are {cycle_period_by_axis}")

        return None

    def show(self):
        print(f"After {self.num_steps} steps:")
        for moon in self.moons:
            print(moon.to_str())

        print()

    def total_energy(self):
        return sum([moon.total_energy() for moon in self.moons])

    @classmethod
    def init_from_str(cls, moon_str):
        moons = []
        for line in moon_str.strip().split("\n"):
            x, y, z = [int(pair.strip().split("=")[1]) for pair in line.strip("<>").split(",")]
            moons.append(Moon(Vector(x, y, z)))

        return System(moons)

    @classmethod
    def init_from_file(cls, filename):
        with open(filename) as f:
            return cls.init_from_str(f.read())

    # for testing only
    @classmethod
    def init_str_and_get_energy(cls, moon_str, num_steps):
        system = System.init_from_str(moon_str)
        system.step(num_steps)
        return system.total_energy()

    # for testing only
    @classmethod
    def init_str_and_determine_cycles(cls, moon_str):
        system = System.init_from_str(moon_str)
        return system.step_and_check_for_cycles()


system1_txt = """
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
"""

assert 179 == System.init_str_and_get_energy(system1_txt, 10)
assert 2772 == System.init_str_and_determine_cycles(system1_txt)


system2_txt = """
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
"""

assert 1940 == System.init_str_and_get_energy(system2_txt, 100)
assert 4686774924 == System.init_str_and_determine_cycles(system2_txt)

SYSTEM = System.init_from_file("data/12.txt")
SYSTEM.step(1000)
print(SYSTEM.total_energy())

SYSTEM = System.init_from_file("data/12.txt")
print(SYSTEM.step_and_check_for_cycles())
