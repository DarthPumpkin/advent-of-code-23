import fileinput
from collections import Counter
from dataclasses import dataclass


_CARD_VALUES = {
    'A': 14,
    'K': 13,
    'Q': 12,
    'J': 1,
    'T': 10,
    **{str(digit): digit for digit in range(2, 10)},}


@dataclass
class Hand:
    hand_str: str

    @property
    def rank(self) -> int:
        if 'J' in self.hand_str:
            candidates = [Hand(self.hand_str.replace('J', s, 1)) for s in '23456789TQKA']
            return max(c.rank for c in candidates)
        hand_dict = Counter(self.hand_str)
        counts = sorted(hand_dict.values())
        if counts == [5]:
            return 7
        elif counts == [1, 4]:
            return 6
        elif counts == [2, 3]:
            return 5
        elif counts == [1, 1, 3]:
            return 4
        elif counts == [1, 2, 2]:
            return 3
        elif counts == [1, 1, 1, 2]:
            return 2
        elif counts == [1, 1, 1, 1, 1]:
            return 1
        else:
            raise ValueError('Invalid hand')
    
    @property
    def sort_key(self) -> tuple[int, int, int, int, int, int]:
        return (self.rank, *(_CARD_VALUES[card] for card in self.hand_str))


def main():
    lines = [line.rstrip('\n') for line in fileinput.input()]
    hands_and_bids = [parse_line(line) for line in lines]
    hands_and_bids.sort(key=lambda x: x[0].sort_key)
    winnings = [bid * (i + 1) for i, (_, bid) in enumerate(hands_and_bids)]
    solution = sum(winnings)
    print(solution)


def parse_line(line: str) -> tuple[Hand, int]:
    """Return the parsed line."""
    hand_str, bid_str = line.split(' ')
    hand = Hand(hand_str)
    bid = int(bid_str)
    return hand, bid


if __name__ == '__main__':
    main()
