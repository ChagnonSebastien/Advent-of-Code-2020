import re


def parse_line(line):
    groups = re.search('([a-z]+) ([1-9])', line).groups()
    return groups[0], int(groups[1])


def part1():
    raw_input = open('input', 'r').read().strip()
    data = [parse_line(line) for line in raw_input.split('\n')]

    horizontal_pos = 0
    depth = 0
    aim = 0

    for verb, value in data:
        if verb == 'forward':
            horizontal_pos += value
            depth += aim * value
        else:
            aim += value * (1 if verb == 'down' else -1)

    return horizontal_pos * depth


if __name__ == '__main__':
    print(part1())

