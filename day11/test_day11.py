from day11.day11 import (
    edges,
    get_neighbors,
    get_seat,
    get_visible_counts,
    new_value,
    one_step,
    one_step_part_2,
    run_until_stable,
    run_until_stable_part_2,
)


STEP_0 = """L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
""".split()

STEP_1 = """#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
""".split()

STEP_2 = """#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
""".split()

FINAL = """#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
""".split()


def test_get_seat():
    assert get_seat(0, 9, STEP_2) == "#"
    assert get_seat(3, 0, STEP_2) == "#"
    assert get_seat(3, 1, STEP_2) == "L"
    assert get_seat(3, 4, STEP_2) == "."


def test_get_neighbors():
    assert get_neighbors(0, 0, STEP_1) == "......##"
    assert get_neighbors(0, 9, STEP_2) == "...#.L#."


def test_new_value():
    assert new_value("#", "...##.L#.") == "#"


def test_one_step():
    assert one_step(STEP_0) == STEP_1
    assert one_step(STEP_1) == STEP_2


def test_run_until_stable():
    assert run_until_stable(STEP_0) == FINAL


P2_EXAMPLE_1 = """.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
""".split()

P2_EXAMPLE_2 = """.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
""".split()


def count_str(counts):
    return "\n".join("".join(str(n) for n in row) for row in counts)


def test_edges():
    expected = {
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    }
    assert set(edges(3, 3)) == expected


def test_get_visible_counts():
    assert get_visible_counts("#.\n.#".split()) == [[1, 0], [0, 1]]
    assert get_visible_counts("#.#\n...\n#.#".split()) == [
        [3, 0, 3],
        [0, 0, 0],
        [3, 0, 3],
    ]
    assert get_visible_counts("###\n###\n###".split()) == [
        [3, 5, 3],
        [5, 8, 5],
        [3, 5, 3],
    ]
    assert get_visible_counts(P2_EXAMPLE_1)[4][3] == 8
    example_2_counts = get_visible_counts(P2_EXAMPLE_2)
    assert example_2_counts[0][1] == 4
    assert example_2_counts[3][3] == 0


PART_2_STEP_0 = """L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
""".split()

PART_2_STEP_1 = """#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
""".split()

PART_2_STEP_2 = """
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
""".split()

PART_2_STEP_N = """#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
""".split()


def test_one_step_part_2():
    assert one_step_part_2(PART_2_STEP_0) == PART_2_STEP_1
    assert one_step_part_2(PART_2_STEP_1) == PART_2_STEP_2


def stable_part_2():
    assert run_until_stable_part_2(PART_2_STEP_0) == PART_2_STEP_N
