from src.day_3 import get_power_consumption, get_life_support_rating

INPUT_BITS = [
    "00100",
    "11110",
    "10110",
    "10111",
    "10101",
    "01111",
    "00111",
    "11100",
    "10000",
    "11001",
    "00010",
    "01010",
]


def test_get_power_consumption():
    assert get_power_consumption(INPUT_BITS) == 198


def test_get_life_support_rating():
    assert get_life_support_rating(INPUT_BITS) == 230
