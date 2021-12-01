def part1():
    raw_input = open("input", "r").read().strip()
    data = [int(line) for line in raw_input.split("\n")]
    pairs = zip(data, data[1:])
    return sum([a < b for a, b in pairs])


if __name__ == '__main__':
    print(part1())

