from src.day_2 import get_position


def test_get_position():
    course = "forward 5,down 5,forward 8,up 3,down 8,forward 2".split(",")

    position = get_position(course)
    assert position == 150
