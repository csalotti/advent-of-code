from typing import List
from functools import reduce
from pathlib import Path


def get_n_increases(depths: List[int]) -> int:
    return reduce(
        lambda x, y: x + y, [int(x < y) for x, y in zip(depths[:-1], depths[1:])]
    )


def get_n_windows_increases(depths: List[int]) -> int:
    increase_indicator = [
        int(sum(depths[i : i + 3]) < sum(depths[i + 1 : i + 4]))
        for i in range(len(depths) - 3)
    ]
    return sum(increase_indicator)


if __name__ == "__main__":
    day_1_input = list(
        map(
            lambda x: int(x),
            Path.cwd().joinpath("inputs/day_1.txt").open("r").readlines(),
        )
    )
    print(get_n_increases(day_1_input))
    print(get_n_windows_increases(day_1_input))
