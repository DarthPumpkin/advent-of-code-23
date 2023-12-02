import fileinput


_AVAILABLE_CUBES = {'red': 12, 'green': 13, 'blue': 14}


def main():
    """Find the first and last digits in each line, and treat them as a two-digit number.
    Sum all of these two-digit numbers.
    The digits are either 0-9 or spelled out (one, two, three, etc.)"""
    sum_ = 0
    for line in fileinput.input():
        line = line.rstrip('\n')
        colon_index = line.find(':')
        game_id = int(line[5:colon_index])
        data_substring = line[colon_index + 2:]
        round_strings = data_substring.split('; ')
        game_is_possible = all(_is_round_possible(round_string) for round_string in round_strings)
        if game_is_possible:
            sum_ += game_id
    print(sum_)


def _is_round_possible(round_string: str):
    """Check if the round string is possible."""
    color_substrings = round_string.split(', ')
    color_counts = [color_substring.split(' ') for color_substring in color_substrings]
    color_counts = [(color, int(count)) for count, color in color_counts]
    return all(count <= _AVAILABLE_CUBES[color] for color, count in color_counts)


if __name__ == '__main__':
    main()
