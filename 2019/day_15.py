from enum import Enum
import math
from collections import namedtuple
from dataclasses import dataclass
from heapq import heappush, heappop
from day_05 import IntcodeComputer, test_intcode_computer

class Direction(Enum):
	NORTH = 1
	SOUTH = 2
	WEST = 3
	EAST = 4

	def opposite(self):
		if self == Direction.NORTH:
			return Direction.SOUTH
		if self == Direction.SOUTH:
			return Direction.NORTH
		if self == Direction.WEST:
			return Direction.EAST
		if self == Direction.EAST:
			return Direction.WEST

	def all_forward(self):
		for direction in list(Direction):
			if direction != self.opposite():
				yield direction

@dataclass(frozen=True)
class Position:
	x: int
	y: int

	def __hash__(self):
		return hash((self.x, self.y))

	def step(self, direction):
		if direction == Direction.NORTH:
			return Position(self.x, self.y - 1)
		elif direction == Direction.SOUTH:
			return Position(self.x, self.y + 1)
		elif direction == Direction.WEST:
			return Position(self.x - 1, self.y)
		elif direction == Direction.EAST:
			return Position(self.x + 1, self.y)


def find_oxygen_system(program_file):
	positions = {}
	oxygen_position = None
	oxygen_steps = None

	program = IntcodeComputer.init_from_file(program_file)
	start_position = Position(0, 0)
	positions[start_position] = 0
	steps_taken = []

	# depth first search
	def step(position, direction):
		nonlocal oxygen_position
		nonlocal oxygen_steps
		nonlocal program
		nonlocal steps_taken
		nonlocal positions

		steps_taken.append(direction)
		new_position = position.step(direction)

		# if this position is already in positions{} and the distance there is less than this,
		# we don't need to re-explore here
		prev_num_steps = positions.get(new_position, math.inf)
		if prev_num_steps <= len(steps_taken):
			# step back
			steps_taken = steps_taken[:-1]
			return

		positions[new_position] = len(steps_taken)
		program.pass_in(direction.value)
		state = program.parse_and_get_next_value()

		# hit oxygen system
		if state == 2:
			oxygen_position = new_position
			if oxygen_steps is None or len(oxygen_steps) > len(steps_taken):
				oxygen_steps = [s for s in steps_taken]

			# go backwards
			program.pass_in(direction.opposite().value)
			program.parse_and_get_next_value()
			steps_taken = steps_taken[:-1]

		# hit a wall
		elif state == 0:
			steps_taken = steps_taken[:-1]

		# clear space
		elif state == 1:
			for new_direction in direction.all_forward():
				step(new_position, new_direction)

			# go backwards
			program.pass_in(direction.opposite().value)
			program.parse_and_get_next_value()
			steps_taken = steps_taken[:-1]


	oxygen_steps = None
	for first_direction in list(Direction):
		program = IntcodeComputer.init_from_file(program_file)
		start_position = Position(0, 0)
		positions[start_position] = 0
		steps_taken = []
		step(start_position, first_direction)

	return oxygen_steps

def time_for_oxygen_to_fill_space(program_file, steps_to_oxygen_system):
	starting_program = IntcodeComputer.init_from_file(program_file)
	starting_position = Position(0, 0)
	for direction in steps_to_oxygen_system:
		starting_program.pass_in(direction.value)
		starting_program.parse_and_get_next_value()
		starting_position = starting_position.step(direction)

	positions = {}
	positions[starting_position] = 0
	steps_taken = []

	# depth first search
	def step(position, direction):
		nonlocal program
		nonlocal steps_taken
		nonlocal positions

		steps_taken.append(direction)
		new_position = position.step(direction)

		# if this position is already in positions{} and the distance there is less than this,
		# we don't need to re-explore here
		prev_num_steps = positions.get(new_position, math.inf)
		if prev_num_steps <= len(steps_taken):
			# step back
			steps_taken = steps_taken[:-1]
			return

		positions[new_position] = len(steps_taken)
		program.pass_in(direction.value)
		state = program.parse_and_get_next_value()

		# hit a wall
		if state == 0:
			steps_taken = steps_taken[:-1]
		else:
			# step everywhere except backwards
			for new_direction in direction.all_forward():
				step(new_position, new_direction)

			# go backwards
			program.pass_in(direction.opposite().value)
			program.parse_and_get_next_value()
			steps_taken = steps_taken[:-1]


	for first_direction in list(Direction):
		steps_taken = []
		program = starting_program
		step(starting_position, first_direction)

	return max(positions.values())



test_intcode_computer()

steps_to_oxygen_system = find_oxygen_system("data/15.txt")
assert(len(steps_to_oxygen_system) == 232)
assert(time_for_oxygen_to_fill_space("data/15.txt", steps_to_oxygen_system) == 320)

