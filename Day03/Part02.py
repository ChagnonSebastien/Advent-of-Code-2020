from math import prod

lines = [line for line in open("Day03/input", "r").read().strip().split("\n")]
def hits(right, down): return sum([lines[j][right * i % len(lines[j])] == "#" for i, j in enumerate(range(0, len(lines), down))])
result = prod([hits(right, down) for right, down in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]])
print(result)