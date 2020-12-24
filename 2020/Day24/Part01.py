import re

lines = [re.findall('se|sw|ne|nw|e|w', line) for line in open("Day24/input", "r").read().strip().split('\n')]
directionalOffsets = {'se': (0, 1), 'sw': (-1, 1), 'e': (1, 0), 'w': (-1, 0), 'ne': (1, -1), 'nw': (0, -1)}
flipped = set()

for line in lines:
    pos = (0, 0)
    for dir in line:
        offset = directionalOffsets.get(dir)
        pos = (pos[0] + offset[0], pos[1] + offset[1])

    if pos in flipped:
        flipped.remove(pos)
    else:
        flipped.add(pos)

print(len(flipped))