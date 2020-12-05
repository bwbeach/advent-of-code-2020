from day5.day5 import decode_seat


def test_decode_seat():
    assert decode_seat("FBFBBFFRLR") == 357
    assert decode_seat("BFFFBBFRRR") == 567
    assert decode_seat("FFFBBBFRRR") == 119
    assert decode_seat("BBFFBBFRLL") == 820
