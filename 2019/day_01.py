"""
day 1: the tyranny of the rocket equation
https://adventofcode.com/2019/day/1
"""

import math


def compute_fuel(mass):
    return max(math.floor(mass / 3.0) - 2, 0)


def compute_fuel_for_fuel(initial_mass):
    fuel = compute_fuel(initial_mass)
    if fuel == 0:
        return fuel

    return fuel + compute_fuel_for_fuel(fuel)


total_fuel = 0
with open("data/01.txt", "r") as f:
    for line in f.readlines():
        mass_str = line.strip()
        try:
            total_fuel += compute_fuel_for_fuel(int(mass_str))
        except ValueError:
            print(f"{mass_str} cannot be parsed to int")

print(total_fuel)
