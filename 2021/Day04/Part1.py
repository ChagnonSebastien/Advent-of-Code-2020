import math

import numpy as np
import re


def parse_sequence(raw_sequence):
    return [int(n) for n in raw_sequence.split(',')]


def parse_board(raw_board):
    return np.array([[int(g) for g in re.findall('([0-9]+)', line)] for line in raw_board])


def part1():
    raw_input = open('input', 'r').read().strip()
    data = raw_input.split('\n')
    sequence = parse_sequence(data[0])
    boards = [parse_board(data[2 + i * 6: 7 + i * 6]) for i in range(math.floor(len(data) / 6))]

    for n in sequence:
        for board in boards:
            for line in board:
                for j in range(len(line)):
                    if line[j] == n:
                        line[j] = 0

            sum_cols = np.sum(board, axis=0)
            sum_rows = np.sum(board, axis=1)
            winner = 0 in sum_rows or 0 in sum_cols
            if winner:
                return np.sum(sum_rows, axis=0) * n


if __name__ == '__main__':
    print(part1())

