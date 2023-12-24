import fileinput
import itertools as it
from typing import Sequence

import numpy as np


def main():
    lines = [line.rstrip('\n') for line in fileinput.input()]
    map, locations = parse_input2(lines)
    solution = solve2(map, locations, nsteps=50)
    shape = solution.shape
    visualization = [len(s) for s in solution.flatten()]
    visualization = np.array(visualization).reshape(shape)
    print(count_locations(solution))
    print(visualization)


def solve2(map: np.ndarray, locations: np.ndarray, nsteps=26501365) -> np.ndarray:
    for _ in range(nsteps):
        locations = step2(map, locations)
    return locations


def count_locations(locations):
    return sum(len(s) for s in locations.flatten())


def solve(map: np.ndarray, locations: set[tuple[int, int]], nsteps=26501365) -> set[tuple[int, int]]:
    for _ in range(nsteps):
        locations = step(map, locations)
    return locations


def step2(map: np.ndarray, locations: np.ndarray) -> np.ndarray:
    """Perform one step of the simulation.
    
    Args:
        input_array: The 2d input array (<U1 dtype).
        locations_array: The array of sets of repetition coordinates.
        
    Returns:
        The new set of possible locations."""
    height, width = map.shape
    new_locations = np.array([set() for _ in range(height * width)], dtype=object).reshape(height, width)
    for i, j in it.product(range(height), range(width)):
        set_ij = locations[i, j]
        if len(set_ij) > 0:
            # Check up
            imod, jmod = (i - 1) % height, j % width
            if map[imod, jmod] == '.':
                if i > 0:
                    new_locations[imod, jmod].update(set_ij)
                else:
                    new_locations[height - 1, jmod].update(set((k - 1, l) for k, l in set_ij))
            # Check right
            imod, jmod = i % height, (j + 1) % width
            if map[imod, jmod] == '.':
                if j < width - 1:
                    new_locations[imod, jmod].update(set_ij)
                else:
                    new_locations[imod, 0].update(set((k, l + 1) for k, l in set_ij))
            # Check down
            imod, jmod = (i + 1) % height, j % width
            if map[imod, jmod] == '.':
                if i < height - 1:
                    new_locations[imod, jmod].update(set_ij)
                else:
                    new_locations[0, jmod].update(set((k + 1, l) for k, l in set_ij))
            # Check left
            imod, jmod = i % height, (j - 1) % width
            if map[imod, jmod] == '.':
                if j > 0:
                    new_locations[imod, jmod].update(set_ij)
                else:
                    new_locations[imod, width - 1].update(set((k, l - 1) for k, l in set_ij))
    return new_locations


def parse_input2(lines: Sequence[str]) -> tuple[np.ndarray, np.ndarray]:
    """Parse the input into a suitable datastructure.
    
    Args:
        lines: The newline-stripped input lines.
        
    Returns:
        map: The 2d map array (<U1 dtype).
        locations: The 2d array of locations (object dtype).
        Each entry is the (python) set of indices of repetitions of that location."""
    array = np.array([list(line) for line in lines])
    height, width = array.shape
    locations = np.array([set() for _ in range(height * width)], dtype=object).reshape(height, width)
    locations[array == 'S'].item().add((0, 0))  # Add start location
    array[array == 'S'] = '.'  # Replace start with garden tile
    return array, locations


def step(input_array: np.ndarray, locations: set[tuple[int, int]]) -> set[tuple[int, int]]:
    """Perform one step of the simulation.
    
    Args:
        input_array: The 2d input array (<U1 dtype).
        locations_array: The set of possible locations.
        
    Returns:
        The new set of possible locations."""
    height, width = input_array.shape
    new_locations = set()
    for i, j in locations:
        # Check up
        if input_array[(i - 1) % height, j % width] == '.':
            new_locations.add((i - 1, j))
        # Check right
        if input_array[i % height, (j + 1) % width] == '.':
            new_locations.add((i, j + 1))
        # Check down
        if input_array[(i + 1) % height, j % width] == '.':
            new_locations.add((i + 1, j))
        # Check left
        if input_array[i % height, (j - 1) % width] == '.':
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
