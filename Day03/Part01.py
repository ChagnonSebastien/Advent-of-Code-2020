lines = [line for line in open("Day03/input", "r").read().strip().split("\n")]
hits = 0
offset = 0
for line in lines:
  if line[offset] == "#":
    hits += 1
  offset += 3
  offset %= len(line)
print(hits)