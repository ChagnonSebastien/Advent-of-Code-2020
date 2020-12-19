from math import prod

lines = [0] + [int(line) for line in open("Day10/input", "r").read().strip().split("\n")]
lines.sort()
neighborDistance = [abs(x - y) for x, y in zip(lines, lines[1:] + [max(lines) + 3])]
separators = [i + 1 for i, l in enumerate(neighborDistance) if l == 3]
groups = [lines[a:b] for a, b in zip([0] + separators, separators)]

def arrangements(group):
    if (len(group) <= 2):
        return 1
    return sum([arrangements(group[i+1:]) for i, l in enumerate(group[1:]) if l - group[0] <= 3])

print(prod([arrangements(g) for g in groups]))
