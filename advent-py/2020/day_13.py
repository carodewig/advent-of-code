def next_bus(curr_time, schedule):
    dep_time = curr_time
    buses = list(filter(None, schedule))

    while True:
        for bus in buses:
            if dep_time % bus == 0:
                return (dep_time - curr_time) * bus

        dep_time += 1


def earliest_sequential_departures(schedule):
    buses = list(filter(lambda x: x[1] is not None, enumerate(schedule)))

    step = 1
    t = 0

    synced_buses = []

    while len(synced_buses) < len(buses):
        t += step

        new_synced_buses = []
        for (offset, bus) in buses:
            if (t + offset) % bus == 0:
                new_synced_buses.append(bus)

        if len(synced_buses) != len(new_synced_buses):
            addtl_buses = set(new_synced_buses).difference(synced_buses)
            for new_bus in addtl_buses:
                step *= new_bus

            synced_buses = list(new_synced_buses)

    return t


def parse_bus_sch(text):
    lines = text.split("\n")
    if lines[0] == "":
        lines = lines[1:]

    curr_time = int(lines[0])
    schedule = [int(x) if x != "x" else None for x in lines[1].split(",")]

    return curr_time, schedule


TEST_BUS_SCH = """
939
7,13,x,x,59,x,31,19
"""

# assert next_bus(*parse_bus_sch(TEST_BUS_SCH)) == 295

BUS_SCH = ""
with open("data/13.txt") as fh:
    BUS_SCH = fh.read()

# assert next_bus(*parse_bus_sch(BUS_SCH)) == 3606

assert earliest_sequential_departures(parse_bus_sch(TEST_BUS_SCH)[1]) == 1068781
assert earliest_sequential_departures([17, None, 13, 19]) == 3417
assert earliest_sequential_departures([67, 7, 59, 61]) == 754018
assert earliest_sequential_departures([67, None, 7, 59, 61]) == 779210
assert earliest_sequential_departures([67, 7, None, 59, 61]) == 1261476
assert earliest_sequential_departures([1789, 37, 47, 1889]) == 1202161486

print(earliest_sequential_departures(parse_bus_sch(BUS_SCH)[1]))
