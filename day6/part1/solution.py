import fileinput
from typing import Sequence

import numpy as np


def main():
    lines = [line.rstrip('\n') for line in fileinput.input()]
    times, distances = [parse_line(line) for line in lines]
    solutions = [solve_race(time, distance) for time, distance in zip(times, distances)]
    prod_ = np.prod(solutions)
    print(prod_)


def parse_line(line: str) -> Sequence[int]:
    """Return the parsed line."""
    parts = [part for part in line.split(' ') if part != '']
    return [int(number) for number in parts[1:]]


def solve_race(total_time: int, record_distance: int) -> int:
    record_time = 0.5 * (total_time - np.sqrt(total_time ** 2 - 4 * record_distance))
    min_time_to_beat = int(np.floor(record_time + 1))
    max_time_to_beat = total_time - min_time_to_beat
    range_ = max_time_to_beat - min_time_to_beat + 1
    return range_

if __name__ == '__main__':
    main()
