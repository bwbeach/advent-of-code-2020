import pytest
from day12.day12 import EAST, WEST, NORTH, SOUTH, Ship1, Ship2, Vector


def test_vector():
    assert Vector(1, 2) == Vector(1, 2)
    assert Vector(1, 2) != Vector(2, 3)
    with pytest.raises(TypeError):
        assert Vector(1, 2) < Vector(3, 4)
    assert Vector(1, 2) + Vector(4, 8) == Vector(5, 10)
    assert Vector(5, 10) - Vector(1, 2) == Vector(4, 8)
    assert Vector(1, 2).rotate_right(0) == Vector(1, 2)
    assert Vector(1, 2).rotate_right(90) == Vector(2, -1)
    assert Vector(1, 2).rotate_right(180) == Vector(-1, -2)
    assert Vector(1, 2).rotate_right(270) == Vector(-2, 1)
    assert Vector(1, 2).rotate_right(-90) == Vector(-2, 1)


def test_direction():
    assert EAST.rotate_right(90) == SOUTH
    assert NORTH.rotate_right(-90) == WEST
    assert SOUTH.rotate_right(270) == EAST


SAMPLE_ACTIONS = """F10
N3
F7
R90
F11
"""


def test_actions_1():
    ship = Ship1()
    for action in SAMPLE_ACTIONS.split():
        ship.action(action)
    assert ship.location == Vector(17, -8)
    assert ship.manhattan_distance() == 25


def test_actions_2():
    ship = Ship2()
    for action in SAMPLE_ACTIONS.split():
        ship.action(action)
    assert ship.location == Vector(214, -72)
    assert ship.manhattan_distance() == 286
