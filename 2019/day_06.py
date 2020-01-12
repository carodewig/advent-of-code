from collections import defaultdict

import attr


@attr.s(slots=True)
class OrbitMap:
    orbit_map = attr.ib()

    def get_parent_for(self, name):
        for possible_parent in self.orbit_map:
            if name in self.orbit_map[possible_parent]:
                return possible_parent

        return ""

    def get_depths(self):
        depths = defaultdict(int)

        def get_depth_for(name):
            if name in depths:
                return depths[name]

            parent = self.get_parent_for(name)

            if parent == "":
                depths[name] = 0
            else:
                depths[name] = 1 + get_depth_for(parent)

            return depths[name]

        for element in self.orbit_map:
            get_depth_for(element)

        return depths

    def count_orbits(self):
        return sum(self.get_depths().values())

    def get_distance_to_santa(self):
        depths = self.get_depths()

        your_orbit = self.get_parent_for("YOU")
        santas_orbit = self.get_parent_for("SAN")

        total_jumps = 0
        while your_orbit != santas_orbit:
            if depths[your_orbit] > depths[santas_orbit]:
                your_orbit = self.get_parent_for(your_orbit)
                total_jumps += 1
            elif depths[your_orbit] < depths[santas_orbit]:
                santas_orbit = self.get_parent_for(santas_orbit)
                total_jumps += 1
            else:
                your_orbit = self.get_parent_for(your_orbit)
                santas_orbit = self.get_parent_for(santas_orbit)
                total_jumps += 2

        return total_jumps

    @classmethod
    def init_from_str(cls, orbit_str):
        orbit_map = defaultdict(list)
        for line in orbit_str.split("\n"):
            if not line:
                continue

            x, y = line.strip().split(")")
            orbit_map[x].append(y)
            if not y in orbit_map:
                orbit_map[y] = list()

        return OrbitMap(orbit_map)

    @classmethod
    def init_from_file(cls, orbit_file):
        with open(orbit_file) as orbits_io:
            return cls.init_from_str(orbits_io.read())


assert OrbitMap.init_from_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L").count_orbits() == 42
assert (
    OrbitMap.init_from_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN").get_distance_to_santa()
    == 4
)

orbit_map = OrbitMap.init_from_file("data/06.txt")
assert orbit_map.count_orbits() == 261306
print(orbit_map.get_distance_to_santa())
