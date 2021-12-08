""" day 22: wizard simulator 20XX """

from copy import copy, deepcopy
from dataclasses import dataclass


@dataclass
class InstantSpell:
    mana_cost: int
    damage: int
    hp: int


@dataclass
class SpellEffect:
    mana_cost: int
    num_turns: int
    value: int


class Character:
    def __init__(self, hp):
        self.hp = hp

    @property
    def armor(self):
        return 0

    def heal(self, points):
        self.hp += points

    def take_damage(self, damage):
        self.hp -= max(1, damage - self.armor)


class Boss(Character):
    def __init__(self, hp, damage):
        super().__init__(hp)
        self.damage = damage


class Player(Character):
    def __init__(self, hp, mana):
        super().__init__(hp)
        self.mana = mana
        self.shield_active = False

    @property
    def armor(self):
        if self.shield_active:
            return SPELLS["Shield"].value

        return 0


class Battlefield:
    def __init__(self, player, boss):
        self.active_effects = {}

        self.player = player
        self.boss = boss

        self.turn_number = 0
        self.player_mana_used = 0

    def apply_actives(self):
        if "Poison" in self.active_effects:
            self.boss.take_damage(SPELLS["Poison"].value)

        if "Recharge" in self.active_effects:
            self.player.mana += SPELLS["Recharge"].value

        for effect in list(self.active_effects.keys()):
            self.active_effects[effect] -= 1
            if self.active_effects[effect] <= 0:
                del self.active_effects[effect]

        if "Shield" in self.active_effects:
            self.player.shield_active = True
        else:
            self.player.shield_active = False

    def take_boss_turn(self):
        self.player.take_damage(self.boss.damage)
        self.turn_number += 1

    def can_cast_spell(self, spell_name):
        if self.player.mana < SPELLS[spell_name].mana_cost:
            return False

        if isinstance(SPELLS[spell_name], SpellEffect) and spell_name in self.active_effects:
            return False

        return True

    def cast_spell(self, spell_name):
        spell = SPELLS[spell_name]
        self.player.mana -= spell.mana_cost
        self.player_mana_used += spell.mana_cost

        if isinstance(spell, InstantSpell):
            self.boss.take_damage(spell.damage)
            self.player.heal(spell.hp)

        else:
            self.active_effects[spell_name] = spell.num_turns

        self.turn_number += 1

    @property
    def player_turn(self):
        return self.turn_number % 2 == 0

    @property
    def battle_over(self):
        return self.player.hp <= 0 or self.boss.hp <= 0

    def clone(self):
        return deepcopy(self)


SPELLS = {
    "Magic Missile": InstantSpell(53, 4, 0),
    "Drain": InstantSpell(73, 2, 2),
    "Shield": SpellEffect(113, 6, 7),
    "Poison": SpellEffect(173, 6, 3),
    "Recharge": SpellEffect(229, 5, 101),
}


# I'm sure there's a more efficient way to do this, certainly the repeated
# `battlefield` copying is inefficient. takes about a minute to run both parts.
def battle(player, boss, hard_difficulty=False):
    starting_battlefield = Battlefield(player, boss)
    min_mana_to_win = float("inf")

    def _battle(battlefield):
        nonlocal min_mana_to_win

        if battlefield.player_mana_used >= min_mana_to_win:
            return

        if hard_difficulty and battlefield.player_turn:
            battlefield.player.hp -= 1

        battlefield.apply_actives()

        if battlefield.battle_over:
            if battlefield.player.hp > 0:
                min_mana_to_win = min(battlefield.player_mana_used, min_mana_to_win)

            return

        if battlefield.player_turn:
            for spell in SPELLS:
                if battlefield.can_cast_spell(spell):
                    new_battlefield = battlefield.clone()
                    new_battlefield.cast_spell(spell)
                    _battle(new_battlefield)

        else:
            battlefield.take_boss_turn()
            _battle(battlefield)

    _battle(starting_battlefield)
    return min_mana_to_win


assert battle(Player(10, 250), Boss(13, 8)) == 226
assert battle(Player(10, 250), Boss(14, 8)) == 641

assert battle(Player(50, 500), Boss(58, 9)) == 1269
assert battle(Player(50, 500), Boss(58, 9), True) == 1309
