from copy import deepcopy
import networkx as nx
import matplotlib.pyplot as plt

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


def isPortal(coordinate, maze):
    if not isWalkable(coordinate, maze):
        return False

    for step in coordinate.adjacent():
        if not isOnMap(step, maze):
            continue

        if isPortalId(maze[step.x][step.y]):
            return True

    return False


def getPortalId(coordinate, maze):
    for direction in [Vector(0, 1), Vector(0, -1), Vector(1, 0), Vector(-1, 0)]:
        if isPortalId(maze[coordinate.x+direction.x][coordinate.y+direction.y]):
            firstCharOffset = 2 if direction.x < 0 or direction.y < 0 else 1
            firstChar = Vector(coordinate.x + firstCharOffset * direction.x, coordinate.y + firstCharOffset * direction.y)
            secondChar = Vector(firstChar.x + abs(direction.x), firstChar.y + abs(direction.y))
            return maze[firstChar.x][firstChar.y] + maze[secondChar.x][secondChar.y]
    
    raise Exception(str(coordinate) + " is not a portal.")


def isPortalId(char):
    return ord(char) >= ord('A') and ord(char) <= ord('Z')


def isOnMap(coordinate, maze):
    if coordinate.x < 0 or coordinate.y < 0:
        return False
    if coordinate.x >= len(maze) or coordinate.y >= len(maze[coordinate.x]):
        return False
    return True


def isWalkable(coordinate, maze):
    if not isOnMap(coordinate, maze):
        return False
    
    return maze[coordinate.x][coordinate.y] == '.'


def displayMap(maze):
    for row in maze:
        print("".join(row))


def isNode(position, maze):
    if not isWalkable(position, maze):
        return False

    if isPortal(position, maze):
        return True

    adjacentPaths = 0
    for step in position.adjacent():
        if isWalkable(step, maze):
            adjacentPaths += 1
    
    return adjacentPaths > 2

maze = [[c for c in row.rstrip('\n')] for row in open("input.txt", "r")]

nodes = []
portals = dict()
start = None
end = None
topLeft = Vector(1000,1000)
bottomRight = Vector(0,0)

for x in range(len(maze)):
    for y in range(len(maze[x])):
        char = maze[x][y]
        point = Vector(x, y)
        
        if isNode(point, maze):
            nodes.append(point)

        if isPortal(point, maze):
            if point.x > bottomRight.x:
                bottomRight = Vector(point.x, bottomRight.y)
            if point.x < topLeft.x:
                topLeft = Vector(point.x, topLeft.y)
            if point.y > bottomRight.y:
                bottomRight = Vector(bottomRight.x, point.y)
            if point.y < topLeft.y:
                topLeft = Vector(topLeft.x, point.y)

            identifier = getPortalId(point, maze)

            if identifier == "AA":
                start = str(x*1000 + y)
            elif identifier == "ZZ":
                end = str(x*1000 + y)
            else:
                if portals.get(identifier) is None:
                    portals.update({identifier: []})
                portals.get(identifier).append(str(x*1000 + y))


neighbors = dict()

for node in nodes:
    identifier = str(node.x*1000+node.y)
    neighbors.update({identifier:dict()})

    for path in node.adjacent():
        if not isWalkable(path, maze):
            continue

        actual = path
        previous = node
        neighbor = None
        i = 1
        while True:
            if isNode(actual, maze):
                neighbor = actual
                break
            
            valid = False
            for step in actual.adjacent():
                if not isWalkable(step, maze) or step == previous:
                    continue

                previous = actual
                actual = step
                i += 1
                valid = True
                break
            
            if valid == False:
                break

        if neighbor is None:
            continue
        
        neighborIdentifier = str(neighbor.x*1000+neighbor.y)
        neighbors.get(identifier).update({neighborIdentifier:i})

G=nx.Graph()
nLayers = 30

for i in range(nLayers):
    layerAddon = str(i).rjust(3, '0')
    for node in neighbors.keys():
        G.add_node(node + layerAddon)

    for (node, data) in neighbors.items():
        for (neighbor, distance) in data.items():
            G.add_edge(node + layerAddon, neighbor + layerAddon, length=distance)


for portalPair in portals.values():
    for (a, b) in [(0,1),(1,0)]:
        coordinate = Vector(int(portalPair[a][:len(portalPair[a])-3]), int(portalPair[a][len(portalPair[a])-3:]))
        outter = coordinate.x == topLeft.x or coordinate.x == bottomRight.x or coordinate.y == topLeft.y or coordinate.y == bottomRight.y

        for i in range(nLayers):
            if i > 0 and outter:
                G.add_edge(portalPair[a]+str(i).rjust(3, '0'), portalPair[b]+str(i-1).rjust(3, '0'), length=1)
            
            if i < nLayers - 1 and not outter:
                G.add_edge(portalPair[a]+str(i).rjust(3, '0'), portalPair[b]+str(i+1).rjust(3, '0'), length=1)


print(nx.shortest_path_length(G, start+"000", end+"000", weight='length'))