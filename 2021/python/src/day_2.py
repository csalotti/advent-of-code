from typing import List
from pathlib import Path


def get_position(course: List[str]) -> int:
    positions = {"horizontal": 0, "vertical": 0}

    position_familly = {"forward": "horizontal", "up": "vertical", "down": "vertical"}
    position_values = {"up": -1, "down": 1, "forward": 1}

    for p in course:
        direction, value = p.split(" ")
        positions[position_familly[direction]] += (
            int(value) * position_values[direction]
        )

    return positions["horizontal"] * positions["vertical"]


def get_position_w_aim(course: List[str]) -> int:
    pass


if __name__ == "__main__":
    day_2_input = Path.cwd().joinpath("inputs/day_2.txt").open("r").readlines()

    print(get_position(day_2_input))
