def part2():
    raw_input = open('input', 'r').read().strip()
    data = [int(line) for line in raw_input.split('\n')]
    windows_elements = zip(data, data[1:], data[2:])
    windows_sum = [sum(elements) for elements in windows_elements]
    pairs = zip(windows_sum, windows_sum[1:])
    return sum([a < b for a, b in pairs])


if __name__ == '__main__':
    print(part2())

