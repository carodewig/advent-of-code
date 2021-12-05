""" day 4: giant squid """

from collections import defaultdict, namedtuple

Location = namedtuple("Location", ["x", "y"])


class BingoBoard:
    def __init__(self, board):
        self.board = board
        self.marked = defaultdict(bool)

    @classmethod
    def init_from_str(cls, board_str):
        board = {}

        for (idxx, line) in enumerate(board_str.split("\n")):
            for (idxy, number) in enumerate(line.strip().split()):
                board[Location(idxx, idxy)] = int(number)

        return cls(board)

    def possible_bingo_indices(self):
        # check horizontals and verticals
        for a in range(5):
            yield [Location(b, a) for b in range(5)]
            yield [Location(a, b) for b in range(5)]

    def all_locations(self):
        for x in range(5):
            for y in range(5):
                yield Location(x, y)

    def bingo(self):
        for indices in self.possible_bingo_indices():
            marked_set = [self.marked[loc] for loc in indices]
            if all(marked_set):
                return True

        return False

    def mark(self, number):
        if number not in self.board.values():
            return

        for loc in self.all_locations():
            if self.board[loc] == number:
                self.marked[loc] = True

    def score(self):
        total = 0
        for loc in self.all_locations():
            if not self.marked[loc]:
                total += self.board[loc]

        return total


class BingoGame:
    def __init__(self, boards, number_draw):
        self.numbers = number_draw
        self.boards = boards

    @classmethod
    def init_from_str(cls, boards_str):
        boards_str_list = boards_str.split("\n")

        numbers = []
        boards = []

        idx = 0
        while idx < len(boards_str_list):
            if not boards_str_list[idx].strip():
                idx += 1
                continue

            if not numbers:
                numbers = [int(x) for x in boards_str_list[idx].split(",")]
                idx += 1
                continue

            boards.append(BingoBoard.init_from_str("\n".join(boards_str_list[idx : idx + 5])))
            idx += 5

        return cls(boards, numbers)

    def get_first_winner(self):
        for number in self.numbers:
            for board in self.boards:
                board.mark(number)
                if board.bingo():
                    return number * board.score()

    def get_last_winner(self):
        winners = [False for _ in range(len(self.boards))]
        for number in self.numbers:
            for (idx, (board, won)) in enumerate(zip(self.boards, winners)):
                if won:
                    continue

                board.mark(number)
                if board.bingo():
                    winners[idx] = True

                    if all(winners):
                        return number * board.score()


TEST_STR = """
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""


TEST_GAME = BingoGame.init_from_str(TEST_STR)
assert TEST_GAME.get_first_winner() == 4512
assert TEST_GAME.get_last_winner() == 1924
with open("data/day_04.txt") as f:
    GAME = BingoGame.init_from_str((f.read()))
    print(GAME.get_first_winner())
    print(GAME.get_last_winner())
