import fileinput

import numpy as np


def main():
    """"""
    sum_ = 0
    lines = [line.rstrip('\n') for line in fileinput.input()]
    input_array = np.array([list(line) for line in lines])
    number_positions = [find_numbers(line) for line in lines]
    for line_idx, line in enumerate(lines):
        for start_idx, end_idx in number_positions[line_idx]:
            if is_surrounded_by_symbol(line_idx, start_idx, end_idx, input_array):
                sum_ += int(''.join(line[start_idx:end_idx]))
    print(sum_)


def find_numbers(line) -> list[tuple[int, int]]:
    """Return the start and end positions of the numbers in the line."""
    line_padded = '.' + line + '.'
    enumerated_zip = tuple(enumerate(zip(line_padded[:-1], line_padded[1:])))
    start_indices = [i for i, (c1, c2) in enumerated_zip if not c1.isdigit() and c2.isdigit()]
    end_indices = [i for i, (c1, c2) in enumerated_zip if c1.isdigit() and not c2.isdigit()]
    return list(zip(start_indices, end_indices))


def is_surrounded_by_symbol(line_idx, start_idx, end_idx, input_array) -> bool:
    """Return whether the number is surrounded by a symbol."""
    padded_input = np.pad(input_array, pad_width=1, mode='constant', constant_values='.')
    sub_array = padded_input[line_idx:line_idx + 3, start_idx:end_idx + 2].flatten()
    return any(c not in '0123456789.' for c in sub_array)

if __name__ == '__main__':
    main()
