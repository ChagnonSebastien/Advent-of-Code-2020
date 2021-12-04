import numpy as np


def parse_line(line):
    return [int(c) for c in line]


def part1():
    raw_input = open('input', 'r').read().strip()
    data = np.array([parse_line(line) for line in raw_input.split('\n')])
    col_sum = list(data.sum(axis=0))
    gamma = int(''.join(['0' if s < len(data) / 2 else '1' for s in col_sum]), 2)
    epsilon = int(''.join(['0' if s > len(data) / 2 else '1' for s in col_sum]), 2)
    return gamma * epsilon


if __name__ == '__main__':
    print(part1())

