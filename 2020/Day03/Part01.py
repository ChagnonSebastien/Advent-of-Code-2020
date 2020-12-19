lines = [line for line in open("Day03/input", "r").read().strip().split("\n")]
hits = sum([line[3 * i % len(line)] == "#" for i, line in enumerate(lines)])
print(hits)