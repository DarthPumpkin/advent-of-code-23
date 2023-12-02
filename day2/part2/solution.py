from collections import defaultdict
import fileinput

import numpy as np


_COLORS = ('red', 'green', 'blue')


def main():
    sum_ = 0
    for line in fileinput.input():
        line = line.rstrip('\n')
        colon_index = line.find(':')
        data_substring = line[colon_index + 2:]
        round_strings = data_substring.split('; ')
        round_dicts = [round_dict(round_string) for round_string in round_strings]
        round_array = np.array([[round_dict[color] for color in _COLORS] for round_dict in round_dicts])
        minimum_set = round_array.max(axis=0)
        set_power = minimum_set.prod()
        sum_ += set_power
    print(sum_)


def round_dict(round_string: str) -> dict[str, int]:
    """Return a dictionary of the round string."""
    color_substrings = round_string.split(', ')
    color_counts = [color_substring.split(' ') for color_substring in color_substrings]
    color_counts = [(color, int(count)) for count, color in color_counts]
    return defaultdict(int, color_counts)


if __name__ == '__main__':
    main()
