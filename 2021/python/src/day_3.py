import builtins
from typing import List
from pathlib import Path
import numpy as np


def nxor(a: bool, b: bool):
    return (a and b) or ((not a) and (not b))


def convert_to_list_of_int(bits_array: List[str]) -> List[List[int]]:
    return list(map(lambda x: list(map(int, x)), bits_array))


def get_bits_avg(bits_array: List[List[int]]) -> List[float]:
    return (np.sum(bits_array, axis=0) / len(bits_array)).tolist()


def select_by_bit_criteria(bits_array: List[List[int]], most_common: bool):
    def recurrsive_loop(sub_bits_array: List[List[int]], position: int = 0):
        if len(sub_bits_array) == 1:
            return sub_bits_array[0]

        criteria_bit = get_bits_avg(sub_bits_array)[position]
        criteria_bit = (
            int(most_common)
            if criteria_bit == 0.5
            else int(nxor((criteria_bit > 0.5), most_common))
        )

        return recurrsive_loop(
            [s for s in sub_bits_array if s[position] == criteria_bit], position + 1
        )

    result = recurrsive_loop(bits_array)

    return int("".join(list(map(str, result))), 2)


def get_power_consumption(bits_array: List[str]) -> int:
    bits_sum = get_bits_avg(convert_to_list_of_int(bits_array))
    gamma_rate = int("".join([str(int(b > 0.5)) for b in bits_sum]), 2)
    epsilon_rate = int("".join([str(int(b < 0.5)) for b in bits_sum]), 2)
    return gamma_rate * epsilon_rate


def get_life_support_rating(bits_array: List[str]) -> int:
    bit_int_array = convert_to_list_of_int(bits_array)
    oxygen_generator_rating = select_by_bit_criteria(bit_int_array, True)
    co2_scrubber_rating = select_by_bit_criteria(bit_int_array, False)
    return oxygen_generator_rating * co2_scrubber_rating


if __name__ == "__main__":
    day_3_input = Path.cwd().joinpath("inputs/day_3.txt").read_text().splitlines()

    print(f"Power Consumption : {get_power_consumption(day_3_input)}")
    print(f"Life Support rating : {get_life_support_rating(day_3_input)}")
