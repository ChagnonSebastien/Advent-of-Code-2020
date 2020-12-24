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

for _ in range(100):
    tilesToCompute = set()
    for tile in flipped:
        for n in directionalOffsets.values():
            tilesToCompute.add((tile[0] + n[0], tile[1] + n[1]))

    newFloor = set()
    for tile in tilesToCompute:
        flippedNeighbors = sum([tile in flipped for tile in [(tile[0] + n[0], tile[1] + n[1]) for n in directionalOffsets.values()]])
        if flippedNeighbors == 2 or (tile in flipped and flippedNeighbors == 1):
            newFloor.add(tile)
    
    flipped = newFloor

print(len(flipped))
