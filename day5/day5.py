from more_itertools import pairwise


def decode_seat(seat_spec: str) -> int:
    """
    Returns the seat ID for the given spec.

    The specs are equivalent to binary numbers, with B and R being
    1 and F and L being 0.
    """
    binary_str = (
        seat_spec.replace("F", "0")
        .replace("L", "0")
        .replace("B", "1")
        .replace("R", "1")
    )
    return int(binary_str, 2)


def get_seat_specs():
    """
    Returns all of the seat specs from all of the boarding passes.
    """
    with open("input.txt") as f:
        return list(f.read().splitlines(keepends=False))


def get_seat_ids():
    """
    Returns the seat IDs from all boarding passes, in order.
    """
    return sorted(decode_seat(spec) for spec in get_seat_specs())


def get_highest_seat_id():
    """
    Returns the highest seat ID from all of the boarding passes.
    """
    return max(get_seat_ids())


def find_my_seat_id():
    """
    Returns the one seat ID that is missing from the list, and
    has the seats before and after it assigned.
    """
    missing_seats = [a + 1 for a, b in pairwise(get_seat_ids()) if a + 2 == b]
    assert len(missing_seats) == 1
    return missing_seats[0]


if __name__ == "__main__":
    print("Highest seat ID:", get_highest_seat_id())
    print("My seat ID:", find_my_seat_id())
