""" day 17: no such thing as too much """


def containers_for(quantity, containers):
    for (i, container) in enumerate(containers):
        if quantity > container:
            for sub_container in containers_for(quantity - container, containers[i + 1 :]):
                if sum(sub_container) == quantity - container:
                    yield [container] + sub_container

        if quantity == container:
            yield [container]


def parse_containers(containers_str):
    containers = []
    for container in containers_str.split():
        try:
            containers.append(int(container.strip()))
        except ValueError:
            pass

    return sorted(containers, reverse=True)


def ways_for_containers_to_make(quantity, containers_str):
    container_combos = list(containers_for(quantity, parse_containers(containers_str)))
    return len(container_combos)


def ways_for_minimum_containers_to_make(quantity, containers_str):
    container_combos = list(containers_for(quantity, parse_containers(containers_str)))
    numbers_of_containers = [len(x) for x in container_combos]
    return numbers_of_containers.count(min(numbers_of_containers))


TEST_CONTAINERS = "20 15 10 5 5"
assert ways_for_containers_to_make(25, TEST_CONTAINERS) == 4
assert ways_for_minimum_containers_to_make(25, TEST_CONTAINERS) == 3

CONTAINERS = ""
with open("data/17.txt") as fh:
    CONTAINERS = fh.read()

assert ways_for_containers_to_make(150, CONTAINERS) == 654
print(ways_for_minimum_containers_to_make(150, CONTAINERS))
