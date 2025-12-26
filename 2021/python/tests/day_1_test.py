from src.day_1 import get_n_increases, get_n_windows_increases

DEPTHS = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]


def test_get_n_increases():
    assert get_n_increases(DEPTHS) == 7


def test_get_n_window_increases():
    assert get_n_windows_increases(DEPTHS) == 5
