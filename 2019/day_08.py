"""
day 8: space image format
https://adventofcode.com/2019/day/8
"""

import numpy as np

def parse_image(img, width, height):
    return np.reshape([int(pix) for pix in img], (-1, height, width))

def parse_image_from_file(filename, width, height):
    with open(filename, "r") as file:
        for line in file.readlines():
            return parse_image(line.strip(), width, height)

def count_vals_in_layer(layer, val):
    return sum([1 if x == val else 0 for x in np.reshape(layer.view(), -1)])

def test_for_corruption(img):
    min_zeros = None

    for layer in img:
        num_zeros = count_vals_in_layer(layer, 0)
        if min_zeros is None or num_zeros < min_zeros:
            value = count_vals_in_layer(layer, 1) * count_vals_in_layer(layer, 2)
            min_zeros = num_zeros

    return value

def decode_image(img):
    decoded = img[0]
    for layer in img[1:]:
        for row in range(len(layer)):
            for col in range(len(layer[row])):
                if decoded[row][col] == 2:
                    decoded[row][col] = layer[row][col]

        # no transparent pixels left so we're done
        if count_vals_in_layer(decoded, 2) == 0:
            break

    return decoded


assert np.array_equal(decode_image(parse_image("0222112222120000", 2, 2)), [[0, 1], [1, 0]])

IMAGE = parse_image_from_file("data/08.txt", 25, 6)
print(test_for_corruption(IMAGE))
print(decode_image(IMAGE))
