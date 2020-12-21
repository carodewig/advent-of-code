""" rambunctious recitation """


def play_game(starting_numbers, turns):
    numbers = {y: x + 1 for x, y in enumerate(starting_numbers)}
    prev = starting_numbers[-1]

    turn_number = len(numbers)
    while turn_number < turns:
        if prev not in numbers:
            new_val = 0
        else:
            new_val = turn_number - numbers[prev]

        numbers[prev] = turn_number

        turn_number += 1
        prev = new_val

    print("done!")
    return prev


assert play_game([0, 3, 6], 10) == 0
assert play_game([0, 3, 6], 2020) == 436
assert play_game([1, 3, 2], 2020) == 1
assert play_game([2, 1, 3], 2020) == 10
assert play_game([1, 2, 3], 2020) == 27
assert play_game([2, 3, 1], 2020) == 78
assert play_game([3, 2, 1], 2020) == 438
assert play_game([3, 1, 2], 2020) == 1836

assert play_game([7, 12, 1, 0, 16, 2], 2020) == 410

# assert play_game([0, 3, 6], 30000000) == 175594
# assert play_game([1, 3, 2], 30000000) == 2578
# assert play_game([2, 1, 3], 30000000) == 3544142
# assert play_game([1, 2, 3], 30000000) == 261214
# assert play_game([2, 3, 1], 30000000) == 6895259
# assert play_game([3, 2, 1], 30000000) == 18
# assert play_game([3, 1, 2], 30000000) == 362

print(play_game([7, 12, 1, 0, 16, 2], 30000000))
