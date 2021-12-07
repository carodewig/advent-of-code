""" day 6: lanternfish """


class FishGenerations(list):
    @classmethod
    def from_str(cls, fish_str):
        # 8 is max internal timer, so there are 9 gens
        ages = [int(x) for x in fish_str.split(",")]
        return cls([ages.count(age) for age in range(9)])

    @property
    def num_fish(self):
        return sum(self)

    def num_fish_after_days(self, num_days):
        self.simulate_time(num_days)
        return self.num_fish

    def simulate_time(self, num_days):
        for _ in range(num_days):
            # pop off new spawns
            fish_at_timer0 = self.pop(0)

            # new fish are born
            self.append(fish_at_timer0)

            # current fish reset their internal clock
            self[6] += fish_at_timer0


TEST_STARTING_FISH = "3,4,3,1,2"

assert FishGenerations.from_str(TEST_STARTING_FISH).num_fish_after_days(18) == 26
assert FishGenerations.from_str(TEST_STARTING_FISH).num_fish_after_days(80) == 5934
assert FishGenerations.from_str(TEST_STARTING_FISH).num_fish_after_days(256) == 26984457539

with open("data/day_06.txt") as f:
    STARTING_FISH = f.read()

assert FishGenerations.from_str(STARTING_FISH).num_fish_after_days(80) == 380758
assert FishGenerations.from_str(STARTING_FISH).num_fish_after_days(256) == 1710623015163
