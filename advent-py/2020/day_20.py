""" jurassic jigsaw """


from enum import Enum
from collections import defaultdict, namedtuple
from copy import copy
import attr


class Pixel(Enum):
    ACTIVE = "#"
    INACTIVE = "."


class NeighborDirection(Enum):
    UP = 0
    LEFT = 1
    DOWN = 2
    RIGHT = 3


Location2d = namedtuple("Location2d", ["x", "y"])


def opposite_direction(direction):
    return NeighborDirection((direction.value + 2) % 4)


def normalize_loc(loc, grid_size):
    x = loc.x
    y = loc.y

    while x < 0:
        x += grid_size

    while y < 0:
        y += grid_size

    return Location2d(x % grid_size, y % grid_size)


@attr.s
class Tile:
    id_num = attr.ib()
    grid = attr.ib()

    grid_size = attr.ib(default=10)
    neighbor_ids = attr.ib(factory=dict)

    def __str__(self):
        return "\n".join(
            [
                "".join([self.grid[Location2d(x, y)].value for x in range(self.grid_size)])
                for y in range(self.grid_size)
            ]
        )

    def rotations(self):
        tile = self
        yield tile

        for _ in range(3):
            tile = tile.rotated_once()
            yield tile

    def rotated(self, n):
        for (i, tile) in enumerate(self.rotations()):
            if i == (n % 4):
                return tile

    def rotated_once(self):
        new_grid = {}
        for loc in self.grid.keys():
            new_loc = self.normalize(Location2d(self.grid_size - 1 - loc.y, loc.x))
            new_grid[new_loc] = self.grid[loc]

        return Tile(self.id_num, new_grid)

    def flipped_x(self):
        new_grid = {}
        for loc in self.grid.keys():
            new_loc = self.normalize(Location2d(self.grid_size - loc.x - 1, loc.y))
            new_grid[new_loc] = self.grid[loc]

        return Tile(self.id_num, new_grid)

    def flipped_y(self):
        new_grid = {}
        for loc in self.grid.keys():
            new_loc = self.normalize(Location2d(loc.x, self.grid_size - loc.y - 1))
            new_grid[new_loc] = self.grid[loc]

        return Tile(self.id_num, new_grid)

    def normalize(self, loc):
        return normalize_loc(loc, self.grid_size)

    def top_edge(self):
        return "".join([self.grid[Location2d(x, 0)].value for x in range(self.grid_size)])

    def edge(self, target_direction):
        for (direction, edge) in self.edges():
            if direction == target_direction:
                return edge

    def edges(self):
        for (rot, tile) in enumerate(self.rotations()):
            yield (NeighborDirection(rot), tile.top_edge())

    def open_edges(self):
        for (direction, edge) in self.edges():
            if direction not in self.neighbor_ids:
                yield (direction, edge)

    def can_place_neighbor(self):
        return self.neighbor_ids.size() < 4

    @classmethod
    def from_str(cls, text):
        lines = text.split("\n")
        while not lines[0]:
            lines.pop(0)

        header = lines.pop(0)
        id_num = int(header.split(" ")[1].strip(":"))

        grid = {}

        for (y, row) in enumerate(lines):
            for (x, pix) in enumerate(row.strip()):
                grid[Location2d(x, y)] = Pixel(pix)

        return cls(id_num, grid)


def read_tiles(tiles_str):
    tiles = []
    lines = tiles_str.strip().split("\n")

    while lines:
        while not lines[0]:
            lines.pop(0)

        one_tile = []
        for _ in range(11):
            one_tile.append(lines.pop(0))

        tiles.append(Tile.from_str("\n".join(one_tile)))

    return tiles


def piece_tiles_together(tiles, write_file=None):
    all_tiles = {tile.id_num: copy(tile) for tile in tiles}
    placed_tile_ids = [list(all_tiles.keys())[0]]

    # def get_corner()

    def remaining_tile_ids():
        for tile_id in all_tiles.keys():
            if tile_id not in placed_tile_ids:
                yield tile_id

    def get_neighbor(tile, direction):
        if direction in tile.neighbor_ids:
            return all_tiles[tile.neighbor_ids[direction]]

    audited = {}

    def audit_neighbors():
        for placed_tile_id in placed_tile_ids:
            placed_tile = all_tiles[placed_tile_id]

            if placed_tile_id not in audited:
                audited[placed_tile_id] = {}

            # see if we've already got all 4 edges
            if len(audited[placed_tile_id]) == 4:
                continue

            for direction1 in [NeighborDirection.UP, NeighborDirection.DOWN]:
                for direction2 in [NeighborDirection.LEFT, NeighborDirection.RIGHT]:
                    if get_neighbor(placed_tile, direction1):
                        audited[placed_tile_id][direction1] = True

                    if get_neighbor(placed_tile, direction2):
                        audited[placed_tile_id][direction2] = True

                    if (
                        get_neighbor(placed_tile, direction2) is None
                        and (n1 := get_neighbor(placed_tile, direction1))
                        and n1 is not None
                        and (n2 := get_neighbor(n1, direction2))
                        and (n3 := get_neighbor(n2, opposite_direction(direction1)))
                    ):
                        placed_tile.neighbor_ids[direction2] = n3.id_num

            for direction1 in [NeighborDirection.LEFT, NeighborDirection.RIGHT]:
                for direction2 in [NeighborDirection.UP, NeighborDirection.DOWN]:
                    if (
                        get_neighbor(placed_tile, direction2) is None
                        and (n1 := get_neighbor(placed_tile, direction1))
                        and (n2 := get_neighbor(n1, direction2))
                        and (n3 := get_neighbor(n2, opposite_direction(direction1)))
                    ):
                        placed_tile.neighbor_ids[direction2] = n3.id_num

    def get_grid_corner(start_tile, up_down, left_right):
        if up_down in start_tile.neighbor_ids:
            return get_grid_corner(get_neighbor(start_tile, up_down), up_down, left_right)
        if left_right in start_tile.neighbor_ids:
            return get_grid_corner(get_neighbor(start_tile, left_right), up_down, left_right)
        return start_tile

    while len(list(remaining_tile_ids())) > 0:
        print("NEW ROUND", len(placed_tile_ids), "/", len(all_tiles))

        placed_this_round = False

        for placed_tile_id in placed_tile_ids:
            placed_tile = all_tiles[placed_tile_id]
            available_edges = list(placed_tile.open_edges())

            if len(available_edges) == 0:
                continue

            for remaining_tile_id in remaining_tile_ids():
                remaining_tile = all_tiles[remaining_tile_id]

                for (placed_dir, placed_edge) in available_edges:
                    for (_, remaining_edge) in remaining_tile.open_edges():
                        # negative of remaining edge is super important!!
                        if placed_edge == remaining_edge[::-1]:
                            target_remaining_dir = opposite_direction(placed_dir)
                            target_tile = remaining_tile
                            while target_tile.edge(target_remaining_dir) != remaining_edge:
                                target_tile = target_tile.rotated_once()

                            placed_tile.neighbor_ids[placed_dir] = target_tile.id_num
                            target_tile.neighbor_ids[target_remaining_dir] = placed_tile.id_num

                            all_tiles[placed_tile.id_num] = placed_tile
                            all_tiles[target_tile.id_num] = target_tile

                            placed_tile_ids.append(remaining_tile_id)
                            placed_this_round = True
                            break

                    if placed_this_round:
                        break

                if placed_this_round:
                    break

                # otherwise, flip it so hopefully it'll fit next time
                all_tiles[remaining_tile_id] = remaining_tile.flipped_x()

            if placed_this_round:
                break

        audit_neighbors()

    def get_as_grid():
        # list of lists -- each inner list = one row
        grid = []
        start_tile = get_grid_corner(
            list(all_tiles.values())[0], NeighborDirection.UP, NeighborDirection.LEFT
        )

        while start_tile:
            row = []
            curr_tile = start_tile

            while curr_tile:
                row.append(curr_tile)
                curr_tile = curr_tile.get_neighbor(NeighborDirection.RIGHT)

            grid.append(row)
            start_tile = get_neighbor(NeighborDirection.DOWN)

        return grid

    if write_file:
        as_grid = get_as_grid()
        for grid_row in as_grid:
            for row in range(grid_row[0].grid_size):
                

    return (
        get_grid_corner(
            list(all_tiles.values())[0], NeighborDirection.UP, NeighborDirection.LEFT
        ).id_num
        * get_grid_corner(
            list(all_tiles.values())[0], NeighborDirection.UP, NeighborDirection.RIGHT
        ).id_num
        * get_grid_corner(
            list(all_tiles.values())[0], NeighborDirection.DOWN, NeighborDirection.LEFT
        ).id_num
        * get_grid_corner(
            list(all_tiles.values())[0], NeighborDirection.DOWN, NeighborDirection.RIGHT
        ).id_num
    )


TEST_TILE_STR = """
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###
"""

TEST_TILE = Tile.from_str(TEST_TILE_STR)

assert TEST_TILE.rotated(2).rotated(2) == TEST_TILE
assert TEST_TILE.flipped_x().flipped_x() == TEST_TILE
assert TEST_TILE.flipped_y().flipped_y() == TEST_TILE
assert TEST_TILE.flipped_x().flipped_y().rotated(2) == TEST_TILE
assert TEST_TILE.rotated(1) != TEST_TILE
assert TEST_TILE.rotated(2) != TEST_TILE

TEST_TILES = []
with open("data/test/20.txt") as fh:
    TEST_TILES = read_tiles(fh.read())

assert piece_tiles_together(TEST_TILES) == 20899048083289


TILES = []
with open("data/20.txt") as fh:
    TILES = read_tiles(fh.read())

assert piece_tiles_together(TILES) == 28057939502729
