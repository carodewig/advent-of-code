"""
day 7: amplification circuit
https://adventofcode.com/2019/day/7
"""

from itertools import permutations
import attr

from day_05 import IntcodeComputer, test_intcode_computer


@attr.s(slots=True)
class AmpSequence:
    amps = attr.ib(factory=list)

    def _reset(self):
        for amp in self.amps:
            amp.reset()

    @classmethod
    def init_from_file(cls, filename):
        return AmpSequence(amps=[IntcodeComputer.init_from_file(filename, replace_stdout=True) for _ in range(5)])

    @classmethod
    def init_from_list(cls, l):
        return AmpSequence(amps=[IntcodeComputer(list(l), replace_stdout=True) for _ in range(5)])

    def get_amplification(self, phases):
        for index in range(len(self.amps)):
            self.amps[index].replace_stdin = [phases[index]]

        last_value = None
        val = 0
        while all([amp.alive for amp in self.amps]):
            for amp in self.amps:
                amp.replace_stdin.append(val)
                val = amp.parse_and_get_first_value()

            if val is not None:
                last_value = val

            if 0 in phases:
                break

        self._reset()
        return last_value


def test_amplifiers():
    assert (
        AmpSequence.init_from_list([3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]).get_amplification(
            [4, 3, 2, 1, 0]
        )
        == 43210
    )
    assert (
        AmpSequence.init_from_list(
            [3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]
        ).get_amplification([0, 1, 2, 3, 4])
        == 54321
    )
    assert (
        AmpSequence.init_from_list(
            [
                3,
                31,
                3,
                32,
                1002,
                32,
                10,
                32,
                1001,
                31,
                -2,
                31,
                1007,
                31,
                0,
                33,
                1002,
                33,
                7,
                33,
                1,
                33,
                31,
                31,
                1,
                32,
                31,
                31,
                4,
                31,
                99,
                0,
                0,
                0,
            ]
        ).get_amplification([1, 0, 4, 3, 2])
        == 65210
    )

    assert (
        AmpSequence.init_from_list(
            [
                3,
                52,
                1001,
                52,
                -5,
                52,
                3,
                53,
                1,
                52,
                56,
                54,
                1007,
                54,
                5,
                55,
                1005,
                55,
                26,
                1001,
                54,
                -5,
                54,
                1105,
                1,
                12,
                1,
                53,
                54,
                53,
                1008,
                54,
                0,
                55,
                1001,
                55,
                1,
                55,
                2,
                53,
                55,
                53,
                4,
                53,
                1001,
                56,
                -1,
                56,
                1005,
                56,
                6,
                99,
                0,
                0,
                0,
                0,
                10,
            ]
        ).get_amplification([9, 7, 8, 5, 6])
        == 18216
    )

    assert (
        AmpSequence.init_from_list(
            [
                3,
                26,
                1001,
                26,
                -4,
                26,
                3,
                27,
                1002,
                27,
                2,
                27,
                1,
                27,
                26,
                27,
                4,
                27,
                1001,
                28,
                -1,
                28,
                1005,
                28,
                6,
                99,
                0,
                0,
                5,
            ]
        ).get_amplification([9, 8, 7, 6, 5])
        == 139629729
    )


if __name__ == "__main__":
    test_intcode_computer()
    test_amplifiers()

    AMPS = AmpSequence.init_from_file("data/07.txt")

    MAX_AMPLIFICATION = -1
    for phase_settings in permutations(range(5)):
        MAX_AMPLIFICATION = max(MAX_AMPLIFICATION, AMPS.get_amplification(phase_settings))
    print(MAX_AMPLIFICATION)

    MAX_AMPLIFICATION_WITH_FEEDBACK = -1
    for phase_settings in permutations(range(5, 10)):
        MAX_AMPLIFICATION_WITH_FEEDBACK = max(MAX_AMPLIFICATION_WITH_FEEDBACK, AMPS.get_amplification(phase_settings))
    print(MAX_AMPLIFICATION_WITH_FEEDBACK)
