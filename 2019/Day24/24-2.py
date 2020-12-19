import networkx as nx
from math import ceil

class Vector:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def __str__(self):
        return( "({:d}, {:d})".format(self.x, self.y))
    
    def __repr__(self):
        return( "({:d}, {:d})".format(self.x, self.y))
    
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def adjacent(self):
        return [Vector(self.x + d.x, self.y + d.y) for d in [Vector(0, 1), Vector(0, -1), Vector(1, 0), Vector(-1, 0)]]

def isOnMap(coordinate):
    if coordinate.x < 0 or coordinate.y < 0:
        return False
    if coordinate.x == 2 and coordinate.y == 2:
        return False
    if coordinate.x >= 5 or coordinate.y >= 5:
        return False
    return True

time = 200
nLevels = ceil(time / 2)

G=nx.Graph()
for level in range(-nLevels, nLevels + 1):

    for i in range(5):
        for j in range(5):
            if i == 2 and j == 2:
                continue

            for neighbor in Vector(i, j).adjacent():
                if not isOnMap(neighbor):
                    continue

                G.add_edge(str(level) + str(i) + str(j), str(level) + str(neighbor.x) + str(neighbor.y))
            
            if i == 0:
                G.add_edge(str(level) + str(i) + str(j), str(level + 1) + str(1) + str(2))
            if i == 4:
                G.add_edge(str(level) + str(i) + str(j), str(level + 1) + str(3) + str(2))
            if j == 0:
                G.add_edge(str(level) + str(i) + str(j), str(level + 1) + str(2) + str(1))
            if j == 4:
                G.add_edge(str(level) + str(i) + str(j), str(level + 1) + str(2) + str(3))


#for tile in G.nodes:
#    print((tile, [n for n in G.neighbors(tile)]))

swarm = dict()

bugs = [module.rstrip('\n') for module in open("input.txt", "r")]
for i in range(5):
    for j in range(5):
        if bugs[i][j] == "#":
            swarm.update({str(0)+str(i)+str(j):True})

while time > 0:
    newSwarm = dict()

    for tile in G.nodes:
        adjacentAlive = 0
        for neighbor in [n for n in G.neighbors(tile)]:
            if swarm.get(neighbor, False):
                adjacentAlive += 1
        
        preinfected = swarm.get(tile, False)
        if adjacentAlive == 1:
            newSwarm.update({tile: True})
        elif adjacentAlive == 2 and not preinfected:
            newSwarm.update({tile: True})

    swarm = newSwarm
    time -= 1

for level in range(-nLevels, nLevels + 1):
    print("LEVEL: " + str(level))
    for i in range(5):
        line = ""
        for j in range(5):
            if i == 2 and j == 2:
                line = line + "?"
                continue
            
            line = line + ("#" if swarm.get(str(level)+str(i)+str(j), False) else ".")

        print(line)
    print("=====================")

print(len(swarm))
            