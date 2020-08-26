"""
day 12: n-body problem
"""

from collections import namedtuple
from itertools import combinations

import attr

Vector = namedtuple('Vector', 'x y z')

def add_vectors(v1, v2):
	return Vector(*[x1 + x2 for x1, x2 in zip(v1, v2)])
def mult_vector(s, v):
	return Vector(*[s * x for x in v])

def sub_vectors(v1, v2):
	return add_vectors(v1, mult_vector(-1, v2))

def norm_delta(v1, v2):
	return Vector(*[int((y - x) / abs(y - x)) if x != y else 0 for x, y in zip(v1, v2)])

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

	prior_states = attr.ib(init=False, factory=list)

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

	def _step(self):
		self.apply_gravity()
		self.apply_velocity()
		self.num_steps += 1

	def step(self, n=1):
		for _ in range(n):
			self._step()

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



system1 = """
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
"""

assert 179 == System.init_str_and_get_energy(system1, 10)


system2 = """
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
"""

assert 1940 == System.init_str_and_get_energy(system2, 100)

SYSTEM = System.init_from_file("data/12.txt")
SYSTEM.step(1000)
print(SYSTEM.total_energy())


# part 2
# look for loops per axis rather than for whole system

