import fileinput

import numpy as np


def main():
    sum_ = 0
    lines = [line.rstrip('\n') for line in fileinput.input()]
    padded_array = np.array([list(line) for line in lines])
    padded_array = np.pad(padded_array, pad_width=1, mode='constant', constant_values='.')
    lines_padded = [''.join(line) for line in padded_array]
    padded_number_positions = [find_numbers(line) for line in lines_padded]
    for line_idx in range(len(lines)):
        gear_ratios = find_gear_ratios(padded_array[line_idx:line_idx + 3],
                                       padded_number_positions[line_idx:line_idx + 3])
        sum_ += sum(gear_ratios)
    print(sum_)


def find_numbers(line_padded) -> list[tuple[int, int]]:
    """Return the start and end positions of the numbers in the line."""
    enumerated_zip = tuple(enumerate(zip(line_padded[:-1], line_padded[1:])))
    start_indices = [i + 1 for i, (c1, c2) in enumerated_zip if not c1.isdigit() and c2.isdigit()]
    end_indices = [i + 1 for i, (c1, c2) in enumerated_zip if c1.isdigit() and not c2.isdigit()]
    return list(zip(start_indices, end_indices))


def find_gear_ratios(padded_sub_array: np.ndarray,
                     padded_number_positions: list[list[tuple[int, int]]]) -> list[int]:
    """Return the gear ratios in the sub array."""
    gear_ratios = []
    star_indices = np.where(padded_sub_array[1] == '*')[0]
    for star_idx in star_indices:
        adjacent_numbers = [[(i, j) for i, j in line_positions if i - 1 <= star_idx <= j] for line_positions in padded_number_positions]
        flat_indices = [(line_idx, *item) for line_idx, sublist in enumerate(adjacent_numbers) for item in sublist]
        flat_numbers = [int(''.join(padded_sub_array[line_idx, start_idx:end_idx])) for line_idx, start_idx, end_idx in flat_indices]
        if len(flat_numbers) == 2:
            gear_ratios.append(flat_numbers[0] * flat_numbers[1])
    return gear_ratios


if __name__ == '__main__':
    main()
