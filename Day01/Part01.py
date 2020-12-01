from itertools import combinations
lines = [int(line) for line in open("Day01/input", "r").read().strip().split("\n")]
answer = next(i * j for (i, j) in combinations(lines, 2) if i + j == 2020)
print(answer)