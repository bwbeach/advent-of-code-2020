from typing import Generator, List, Tuple


def get_seat(r: int, c: int, seats: List[str]):
    """
    Returns the seat at row 'r' and column 'c'.
    Defaults to "." if either or both indices are out of bounds.
    """
    if 0 <= r < len(seats) and 0 <= c < len(seats[0]):
        return seats[r][c]
    else:
        return "."


def get_neighbors(row: int, column: int, seats: List[str]):
    """
    Returns the eight (or fewer) neighbor cells of a given cell
    """
    return "".join(
        get_seat(r, c, seats)
        for r in range(row - 1, row + 2)
        for c in range(column - 1, column + 2)
        if r != row or c != column
    )


def new_value(seat: str, neighbors: str):
    """
    Returns the new state of one seat.
    """
    occupied_count = neighbors.count("#")
    if seat == "L" and occupied_count == 0:
        return "#"
    elif seat == "#" and 4 <= occupied_count:
        return "L"
    else:
        return seat


def one_step(seats: List[str]) -> List[str]:
    """
    Returns a new seat matrix after applying the rule to each seat once.
    """
    return list(
        "".join(
            new_value(get_seat(r, c, seats), get_neighbors(r, c, seats))
            for c in range(len(seats[0]))
        )
        for r in range(len(seats))
    )


def run_until_stable(seats: List[str]) -> Tuple[List[str], int]:
    """
    Runs the simulation until it stabilizes.
    """
    iterations = 0
    while True:
        new_seats = one_step(seats)
        iterations += 1
        if new_seats == seats:
            return seats, iterations
        seats = new_seats


def edges(
    rc: int, cc: int
) -> Generator[Tuple[Tuple[int, int], List[Tuple[int, int]]], None, None]:
    """
    Yields all of the cells at the edge of a grid of a given size,
    along with the list of three directions you can go from each
    cell.
    """
    # Indices of the four sides
    top = 0
    bottom = rc - 1
    left = 0
    right = cc - 1

    # Four corners
    yield (top, left), [(1, 0), (1, 1), (0, 1)]
    yield (top, right), [(1, 0), (1, -1), (0, -1)]
    yield (bottom, left), [(-1, 0), (-1, 1), (0, 1)]
    yield (bottom, right), [(-1, 0), (-1, -1), (0, -1)]

    # Four edges, excluding corners
    for r in range(top + 1, bottom):
        yield (r, left), [(-1, 1), (0, 1), (1, 1)]
        yield (r, right), [(-1, -1), (0, -1), (1, -1)]
    for c in range(left + 1, right):
        yield (top, c), [(1, -1), (1, 0), (1, 1)]
        yield (bottom, c), [(-1, -1), (-1, 0), (-1, 1)]


def get_visible_counts(seats: List[str]) -> List[List[int]]:
    """
    Returns a matrix the same shape as seats where every cell
    that is not a seat contains 0, and every cell that is a
    seat contains the number of occupied seats visible from
    that seat.
    """
    # Start with all counters at 0
    rc = len(seats)
    cc = len(seats[0])
    result = [[0 for _c in range(cc)] for _r in range(rc)]

    # Scan in all directions from each edge cell
    for (edge, directions) in edges(rc, cc):
        for dr, dc in directions:
            r, c = edge
            prev = seats[r][c]
            r += dr
            c += dc
            while 0 <= r < rc and 0 <= c < cc:
                seat = seats[r][c]
                if seat != ".":
                    if prev == "#":
                        result[r][c] += 1
                    prev = seat
                r += dr
                c += dc

    return result


def new_value_part_2(seat: str, visible_count: int) -> str:
    """
    Returns the next state for one seat.
    """
    if seat == "L" and visible_count == 0:
        return "#"
    elif seat == "#" and 5 <= visible_count:
        return "L"
    else:
        return seat


def one_step_part_2(seats: List[str]) -> List[str]:
    """
    Takes a seat matrix, and returns a new seat matrix after one
    iteration of people deciding to move.
    """
    rc = len(seats)
    cc = len(seats[0])
    visible_counts = get_visible_counts(seats)
    return [
        "".join(new_value_part_2(seats[r][c], visible_counts[r][c]) for c in range(cc))
        for r in range(rc)
    ]


def run_until_stable_part_2(seats: List[str]) -> Tuple[List[str], int]:
    """
    Runs the simulation until it stabilizes.
    """
    iterations = 0
    while True:
        new_seats = one_step_part_2(seats)
        iterations += 1
        if new_seats == seats:
            return seats, iterations
        seats = new_seats


def main():
    with open("input.txt") as f:
        starting_seats = f.read().split()
    ending_seats, iter_part_1 = run_until_stable(starting_seats)
    print("Part 1:", "".join(ending_seats).count("#"), iter_part_1)
    ending_seats_part_2, iter_part_2 = run_until_stable_part_2(starting_seats)
    print("Part 2:", "".join(ending_seats_part_2).count("#"), iter_part_2)
    print(ending_seats == ending_seats_part_2)


if __name__ == "__main__":
    main()
