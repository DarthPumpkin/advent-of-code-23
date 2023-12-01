import fileinput


def main():
    sum_ = 0
    for line in fileinput.input():
        line = line.rstrip('\n')
        digits = '0123456789'
        digit_1 = next(c for c in line if c in digits)
        digit_2 = next(c for c in line[::-1] if c in digits)
        num = int(digit_1 + digit_2)
        sum_ += num
    print(sum_)


if __name__ == '__main__':
    main()
