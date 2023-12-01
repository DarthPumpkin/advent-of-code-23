import fileinput
import numpy as np


def main():
    """Find the first and last digits in each line, and treat them as a two-digit number.
    Sum all of these two-digit numbers.
    The digits are either 0-9 or spelled out (one, two, three, etc.)"""
    substrings = list('0123456789') + ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
    values = {'0': 0, '1': 1, '2': 2, '3': 3, '4': 4,
            '5': 5, '6': 6, '7': 7, '8': 8, '9': 9,
            'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five': 5,
            'six': 6, 'seven': 7, 'eight': 8, 'nine': 9}
    sum_ = 0
    for line in fileinput.input():
        line = line.rstrip('\n')
        first_indices = [line.find(s) for s in substrings]
        first_indices = [(i if i != -1 else np.nan) for i in first_indices]
        last_indices = [line.rfind(s) for s in substrings]
        last_indices = [(i if i != -1 else np.nan) for i in last_indices]
        substr_index_of_first_digit = np.nanargmin(first_indices, )
        substr_index_of_last_digit = np.nanargmax(last_indices)
        first_substring = substrings[substr_index_of_first_digit]
        last_substring = substrings[substr_index_of_last_digit]
        digit_1 = values[first_substring]
        digit_2 = values[last_substring]
        num = 10 * digit_1 + digit_2
        sum_ += num
    print(sum_)


if __name__ == '__main__':
    main()
