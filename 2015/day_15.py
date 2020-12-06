import re
from collections import namedtuple
from itertools import combinations_with_replacement

import attr

Ingredient = namedtuple("Ingredient", "capacity durability flavor texture calories")


def parse_ingredients(ingredients_str):
    pattern = re.compile(
        r"([A-z]+): capacity ([0-9\-]+), durability ([0-9\-]+), flavor ([0-9\-]+), texture ([0-9\-]+), calories ([0-9\-]+)"
    )
    ingredients = {}

    for ingredient_str in ingredients_str.split("\n"):
        if match := re.match(pattern, ingredient_str):
            groups = match.groups()
            ingredients[groups[0]] = Ingredient(*[int(g) for g in groups[1:]])

    return ingredients


def cookie_score(ingredients, calorie_match=False):
    calorie_count = sum([quantity * ingredient.calories for ingredient, quantity in ingredients])
    if calorie_match and calorie_count != 500:
        return 0

    result = 1
    for prop in ["capacity", "durability", "flavor", "texture"]:
        result *= sum([quantity * getattr(ingredient, prop) for ingredient, quantity in ingredients])
        if result <= 0:
            return 0

    return result


TEST_INGREDIENTS = parse_ingredients(
    """
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"""
)
assert cookie_score([(TEST_INGREDIENTS["Butterscotch"], 44), (TEST_INGREDIENTS["Cinnamon"], 56)]) == 62842880


with open("data/15.txt") as fh:
    INGREDIENTS = parse_ingredients(fh.read())

BEST_SCORE = 0
for _ingredient_list in combinations_with_replacement(INGREDIENTS.values(), 100):
    ingredient_list = [(x, _ingredient_list.count(x)) for x in set(_ingredient_list)]
    score = cookie_score(ingredient_list, calorie_match=True)

    if score > BEST_SCORE:
        BEST_SCORE = score

print(BEST_SCORE)
