import numpy as np


def parse_line(line):
    return [int(c) for c in line]


def filter_data(data, keep_max):
    for i in range(len(data[0])):
        amount = np.sum(data[:, [i]], axis=0)[0]

        if keep_max:
            keep_value = 0 if amount < len(data) / 2 else 1
        else:
            keep_value = 1 if amount < len(data) / 2 else 0

        data = np.array([n for n in data if n[i] == keep_value])
        if len(data) == 1:
            break

    return int(''.join([str(n) for n in data[0]]), 2)


def part2():
    raw_input = open('input', 'r').read().strip()
    data = [parse_line(line) for line in raw_input.split('\n')]

    oxygen_generator_rating = filter_data(np.array(data), 1)
    co2_scrubber_rating = filter_data(np.array(data), 0)
    return oxygen_generator_rating * co2_scrubber_rating


if __name__ == '__main__':
    print(part2())

