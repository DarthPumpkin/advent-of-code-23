import fileinput
from typing import Sequence

import numpy as np


def main():
    lines = [line.rstrip('\n') for line in fileinput.input()]
    map, locations = parse_input(lines)
    solution = solve(map, locations, nsteps=64)
    print(len(solution))


def solve(map: np.ndarray, locations: set[tuple[int, int]], nsteps=64) -> set[tuple[int, int]]:
    for _ in range(nsteps):
        locations = step(map, locations)
    return locations


def step(input_array: np.ndarray, locations: set[tuple[int, int]]) -> set[tuple[int, int]]:
    """Perform one step of the simulation.
    
    Args:
        input_array: The 2d input array (<U1 dtype).
        locations_array: The set of possible locations.
        
    Returns:
        The new set of possible locations."""
    height, width = input_array.shape
    new_locations = set()
    for location in locations:
        i, j = location
        # Check up
        if i > 0 and input_array[i - 1, j] == '.':
            new_locations.add((i - 1, j))
        # Check right
        if j < width - 1 and input_array[i, j + 1] == '.':
            new_locations.add((i, j + 1))
        # Check down
        if i < height - 1 and input_array[i + 1, j] == '.':
            new_locations.add((i + 1, j))
        # Check left
        if j > 0 and input_array[i, j - 1] == '.':
            new_locations.add((i, j - 1))
    return new_locations


def parse_input(lines: Sequence[str]) -> tuple[np.ndarray, set[tuple[int, int]]]:
    array = np.array([list(line) for line in lines])
    locations = np.where(array == 'S')
    locations = [(locations[0].item(), locations[1].item())]
    locations = set(locations)
    array[array == 'S'] = '.'  # Replace start with garden tile
    return array, locations


if __name__ == '__main__':
    main()
