lines = [line for line in open("Day17/input", "r").read().strip().split("\n")]
active = set()
for i in range(len(lines)):
    for j in range(len(lines[0])):
        if lines[i][j] == '#':
            active.add((i, j, 0))

neighbors = []
for x in [-1, 0, 1]:
    for y in [-1, 0, 1]:
        for z in [-1, 0, 1]:
                if x != 0 or y != 0 or z != 0:
                    neighbors.append((x, y, z))

for n in range(1, 7):
    newActive = set()
    for i in range(1-n, len(lines) + n):
        for j in range(1-n, len(lines) + n):
            for k in range(-n, n+1):
                    pos = (i, j, k)
                    adjacents = [(i+x, j+y, k+z) for x, y, z in neighbors]
                    activeNeighbors = sum([1 for a in adjacents if a in active])
                    if activeNeighbors == 3 or (pos in active and activeNeighbors == 2):
                        newActive.add(pos)
    
    active = newActive

print(len(active))
