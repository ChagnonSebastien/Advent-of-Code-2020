lines = [int(line) for line in open("Day10/input", "r").read().strip().split("\n")]
lines.sort()
neighborDistance = [abs(x - y) for x, y in zip([0] + lines, lines + [max(lines) + 3])]
print(sum([n == 1 for n in neighborDistance]) * sum([n == 3 for n in neighborDistance]))