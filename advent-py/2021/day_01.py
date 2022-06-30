""" day 1: sonar sweep """


def count_increasing_depths(measurements):
    prev_measurement = None
    increases = 0

    for measurement in measurements:
        if prev_measurement is not None and measurement > prev_measurement:
            increases += 1

        prev_measurement = measurement

    return increases


def overlay_sliding_window(measurements):
    overlaid = zip(measurements, [None] + measurements, [None, None] + measurements)
    for tup in overlaid:
        if None in tup:
            continue

        yield sum(tup)


def get_measurements(filename):
    with open(filename) as f:
        return [int(line.strip()) for line in f if line]


TEST_MEASUREMENTS = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]

assert count_increasing_depths(TEST_MEASUREMENTS) == 7
assert count_increasing_depths(overlay_sliding_window(TEST_MEASUREMENTS)) == 5

print(count_increasing_depths(get_measurements("data/day_01.txt")))
print(count_increasing_depths(overlay_sliding_window(get_measurements("data/day_01.txt"))))
