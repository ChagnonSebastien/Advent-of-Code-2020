import re
import numpy as np

data = open("Day20/input", "r").read().replace('#', '1').replace('.', '0')
lines = [line for line in data.split("\n")]
splits = [i for i, line in enumerate(lines) if line == '']
tiles = [(int(re.search('[0-9]+', lines[a+1]).group()), lines[a+2:b]) for a, b in zip([-1] + splits, splits)]

map = dict()
map.update({ (0, 0): tiles[0][0:2] })
foundTiles = set([tiles[0][0]])

def fill_neighbors(x, y):
    localID, localMap = map.get((x, y))
    localMap = np.array([[c for c in line] for line in localMap])

    for n, a in tiles:
        if n in foundTiles or n == localID:
            continue

        for flip in [True, False]:
            for rot in [0, 1, 2, 3]:
                transformed_tile = np.rot90(np.array([[c for c in line] for line in a]), k=rot)
                if flip:
                    transformed_tile = np.flip(transformed_tile, 0)
                
                if (x, y-1) not in map.keys():
                    if np.array_equal(localMap[0,:], transformed_tile[9,:]):
                        map.update({ (x, y-1): (n, [''.join(line) for line in transformed_tile]) })
                        foundTiles.add(n)
                        fill_neighbors(x, y-1)
                if (x, y+1) not in map.keys():
                    if np.array_equal(localMap[9,:], transformed_tile[0,:]):
                        map.update({ (x, y+1): (n, [''.join(line) for line in transformed_tile]) })
                        foundTiles.add(n)
                        fill_neighbors(x, y+1)
                if (x-1, y) not in map.keys():
                    if np.array_equal(localMap[:,0], transformed_tile[:,9]):
                        map.update({ (x-1, y): (n, [''.join(line) for line in transformed_tile]) })
                        foundTiles.add(n)
                        fill_neighbors(x-1, y)
                if (x+1, y) not in map.keys():
                    if np.array_equal(localMap[:,9], transformed_tile[:,0]):
                        map.update({ (x+1, y): (n, [''.join(line) for line in transformed_tile]) })
                        foundTiles.add(n)
                        fill_neighbors(x+1, y)

fill_neighbors(0, 0)

u = min([y for _, y in map.keys()])
d = max([y for _, y in map.keys()])
r = max([x for x, _ in map.keys()])
l = min([x for x, _ in map.keys()])
print(map.get((r, u))[0] * map.get((r, d))[0] * map.get((l, u))[0] * map.get((l, d))[0])
