lines = [line for line in open("Day03/input", "r").read().strip().split("\n")]
hits = 1
for right, down in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
  offset = 0
  slopeHits = 0
  for i, line in enumerate(lines):
    if i % down == 0:
      if line[offset] == "#":
        slopeHits += 1
      offset += right
      offset %= len(line)
  hits *= slopeHits
print(hits)