""" day 21: rpg simulator 20XX """

from collections import namedtuple
import itertools

Equipment = namedtuple("Equipment", ["cost", "damage", "armor"])


class Character:
    def __init__(self, hp, damage, armor):
        self.max_hp = hp
        self.hp = hp
        self.damage = damage
        self.armor = armor

    def net_damage(self, other_char):
        return max(1, self.damage - other_char.armor)

    def reset(self):
        self.hp = self.max_hp


def battle(player, boss):
    try:
        # returns bool for whether player wins battle
        player_damage = player.net_damage(boss)
        boss_damage = boss.net_damage(player)

        turn = 0
        while player.hp > 0 and boss.hp > 0:
            if turn % 2 == 0:
                boss.hp -= player_damage
            elif turn % 2 == 1:
                player.hp -= boss_damage

            turn += 1

        return player.hp > 0

    finally:
        player.reset()
        boss.reset()


def equipment_permutations():
    for (num_weapons, num_armor, num_rings) in itertools.product([1], [0, 1], [0, 1, 2]):
        for (weapon, armor, rings) in itertools.product(
            itertools.permutations(WEAPONS, num_weapons),
            itertools.permutations(ARMORS, num_armor),
            itertools.permutations(RINGS, num_rings),
        ):
            yield weapon + armor + rings


def min_gold_to_win(boss):
    min_gold = float("inf")

    for equipment in equipment_permutations():
        gold = sum([x.cost for x in equipment])

        if gold > min_gold:
            continue

        damage = sum([x.damage for x in equipment])
        armor = sum([x.armor for x in equipment])

        if battle(Character(100, damage, armor), boss):
            min_gold = gold

        boss.reset()

    return min_gold


def max_gold_to_lose(boss):
    max_gold = float("-inf")

    for equipment in equipment_permutations():
        gold = sum([x.cost for x in equipment])

        if gold < max_gold:
            continue

        damage = sum([x.damage for x in equipment])
        armor = sum([x.armor for x in equipment])

        if not battle(Character(100, damage, armor), boss):
            max_gold = gold

        boss.reset()

    return max_gold


WEAPONS = [Equipment(8, 4, 0), Equipment(10, 5, 0), Equipment(25, 6, 0), Equipment(40, 7, 0), Equipment(74, 8, 0)]
ARMORS = [Equipment(13, 0, 1), Equipment(31, 0, 2), Equipment(53, 0, 3), Equipment(75, 0, 4), Equipment(102, 0, 5)]
RINGS = [
    Equipment(25, 1, 0),
    Equipment(50, 2, 0),
    Equipment(100, 3, 0),
    Equipment(20, 0, 1),
    Equipment(40, 0, 2),
    Equipment(80, 0, 3),
]

BOSS = Character(109, 8, 2)

assert battle(Character(8, 5, 5), Character(12, 7, 2))

assert min_gold_to_win(BOSS) == 111
assert max_gold_to_lose(BOSS) == 188
