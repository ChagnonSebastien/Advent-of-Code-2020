from itertools import combinations
lines = [int(line) for line in open("Day01/input", "r").read().strip().split("\n")]
answer = next(i * j * k for (i, j, k) in combinations(lines, 3) if i + j + k == 2020)
print(answer)