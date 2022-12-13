import numpy as np
import re
import math


def parse_sequence(raw_sequence):
    return [int(n) for n in raw_sequence.split(',')]


def parse_board(raw_board):
    return np.array([[int(g) for g in re.findall('([0-9]+)', line)] for line in raw_board])


def part2():
    raw_input = open('input', 'r').read().strip()
    data = raw_input.split('\n')
    sequence = parse_sequence(data[0])
    boards = [parse_board(data[2 + i * 6: 7 + i * 6]) for i in range(math.floor(len(data) / 6))]
    checked_boards = np.zeros((len(boards), 5, 5))

    for n in reversed(sequence):
        to_remove = set()

        for b in range(len(checked_boards)):
            for i in range(5):
                for j in range(5):
                    if boards[b][i][j] == n:
                        checked_boards[b][i][j] = 1

            sum_cols = np.sum(checked_boards[b], axis=0)
            sum_rows = np.sum(checked_boards[b], axis=1)
            looser = 0 not in sum_cols and 0 not in sum_rows
            if looser:
                s = 0
                for i in range(5):
                    for j in range(5):
                        if checked_boards[b][i][j] == 1 and boards[b][i][j] != n:
                            s += boards[b][i][j]
                return s * n


if __name__ == '__main__':
    print(part2())

