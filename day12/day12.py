class Vector:
    """
    Two-dimensional vector with integer components
    """

    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, o: object) -> bool:
        if isinstance(o, Vector):
            return self.x == o.x and self.y == o.y
        else:
            return NotImplemented

    def __add__(self, o: object) -> "Vector":
        if isinstance(o, Vector):
            return Vector(self.x + o.x, self.y + o.y)
        else:
            return NotImplemented

    def __sub__(self, o: object) -> "Vector":
        if isinstance(o, Vector):
            return self + (-o)
        else:
            return NotImplemented

    def __mul__(self, o: object) -> "Vector":
        if isinstance(o, int):
            return Vector(self.x * o, self.y * o)
        else:
            return NotImplemented

    def __neg__(self) -> "Vector":
        return Vector(-self.x, -self.y)

    def __repr__(self):
        return "Vector(%d, %d)" % (self.x, self.y)

    def rotate_right(self, degrees: int) -> "Vector":
        degrees = degrees % 360
        if degrees == 0:
            return self
        elif degrees == 90:
            return Vector(self.y, -self.x)
        elif degrees == 180:
            return Vector(-self.x, -self.y)
        elif degrees == 270:
            return Vector(-self.y, self.x)
        else:
            raise ValueError("unsupported rotation: %d" % (degrees,))


NORTH = Vector(0, 1)
EAST = Vector(1, 0)
SOUTH = Vector(0, -1)
WEST = Vector(-1, 0)


NAME_TO_DIRECTION = {
    "N": NORTH,
    "E": EAST,
    "S": SOUTH,
    "W": WEST,
}


class Ship1:
    def __init__(self):
        self.direction = EAST
        self.location = Vector(0, 0)

    def action(self, action: str) -> None:
        code = action[0]
        number = int(action[1:])
        assert 0 < number
        if code == "N":
            self.location += NORTH * number
        elif code == "E":
            self.location += EAST * number
        elif code == "S":
            self.location += SOUTH * number
        elif code == "W":
            self.location += WEST * number
        elif code == "R":
            self.direction = self.direction.rotate_right(number)
        elif code == "L":
            self.direction = self.direction.rotate_right(-number)
        elif code == "F":
            self.location += self.direction * number
        else:
            raise Exception("Unknown action: " + action)

    def manhattan_distance(self):
        return abs(self.location.x) + abs(self.location.y)

    def __repr__(self):
        return "Ship1(%s, %s)" % (
            self.location,
            self.direction,
        )


class Ship2:
    def __init__(self):
        self.location = Vector(0, 0)
        self.waypoint = Vector(10, 1)

    def action(self, action: str) -> None:
        code = action[0]
        number = int(action[1:])
        assert 0 < number
        if code == "N":
            self.waypoint += NORTH * number
        elif code == "E":
            self.waypoint += EAST * number
        elif code == "S":
            self.waypoint += SOUTH * number
        elif code == "W":
            self.waypoint += WEST * number
        elif code == "R":
            self.waypoint = self.waypoint.rotate_right(number)
        elif code == "L":
            self.waypoint = self.waypoint.rotate_right(-number)
        elif code == "F":
            self.location += self.waypoint * number
        else:
            raise Exception("Unknown action: " + action)

    def manhattan_distance(self):
        return abs(self.location.x) + abs(self.location.y)

    def __repr__(self):
        return "Ship2(%s, %s)" % (
            self.location,
            self.waypoint,
        )


def run_ship(ship):
    with open("input.txt") as f:
        for line in f.read().splitlines(keepends=False):
            ship.action(line)
    return ship.manhattan_distance()


def main():
    print("Part 1:", run_ship(Ship1()))
    print("Part 2:", run_ship(Ship2()))


if __name__ == "__main__":
    main()
