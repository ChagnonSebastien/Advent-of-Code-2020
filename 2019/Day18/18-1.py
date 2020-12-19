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

class Node:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    

def isKey(char):
    return ord(char) >= ord('a') and ord(char) <= ord('z')
    

def isDoor(char):
    return ord(char) >= ord('A') and ord(char) <= ord('Z')


def getKey(door):
    return chr(ord(door) + (ord('a') - ord('A')))


def getDoor(key):
    return chr(ord(key) + (ord('A') - ord('a')))


def isOnMap(coordinate, baseMap):
    if coordinate.x < 0 or coordinate.y < 0:
        return False
    if coordinate.x >= len(baseMap) or coordinate.y >= len(baseMap[coordinate.x]):
        return False
    return True


def isWalkable(coordinate, baseMap):
    if not isOnMap(coordinate, baseMap):
        return False
    
    return baseMap[coordinate.x][coordinate.y] != '#'


def accessible(position, G):
    subG = G.copy()
    for n in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
        if subG.has_node(n):
            subG.remove_node(n)
    
    return list(filter(lambda n: len(n) == 1, nx.descendants(subG, position)))


def displayMap(baseMap):
    for row in baseMap:
        print("".join(row))
    

def travel(fromPosition, toPosition, baseMap):
    return 1


def isNode(position, baseMap):
    if not isWalkable(position, baseMap):
        return False

    char = baseMap[position.x][position.y]
    if isKey(char) or isDoor(char):
        return True

    adjacentPaths = 0
    for step in position.adjacent():
        if isWalkable(step, baseMap):
            adjacentPaths += 1
    
    return adjacentPaths > 2


def solve(position, G, layer=0, display=""):
    distances = []
    accessibleKeys = accessible(position, G)

    if len(accessibleKeys) == 0:
        return 0

    i = 0
    for key in accessibleKeys:
        if layer < 20:
            print(display)

        distance = nx.shortest_path_length(G, position, key, weight='length')
        
        newG = G.copy()
        if newG.has_node(getDoor(key)):
            disolve(getDoor(key), newG)
        newG = nx.relabel_nodes(newG, {key: key+'OLD'})

        distances.append(distance + solve(key+'OLD', newG, layer + 1, display + "-" + str(i)))
        i += 1
    
    distances.sort()
    return distances[0]

def disolve(node, G):
    (a, b) = G.adj.get(node)
    G.add_edge(a, b, length=(G.adj.get(node).get(a).get('length')+G.adj.get(node).get(b).get('length')))
    G.remove_node(node)

baseMap = [[c for c in row.rstrip('\n')] for row in open("input.txt", "r")]

position = None
nodes = []

for x in range(len(baseMap)):
    for y in range(len(baseMap[x])):
        char = baseMap[x][y]
        point = Vector(x, y)

        if char == "@":
            position = str(point.x * 100 + point.y)
        
        if isNode(point, baseMap):
            nodes.append(point)

neighbors = dict()

for node in nodes:
    char = baseMap[node.x][node.y]
    identifier = char if isKey(char) or isDoor(char) else str(node.x*100+node.y)
    neighbors.update({identifier:dict()})

    for path in node.adjacent():
        if not isWalkable(path, baseMap):
            continue

        actual = path
        previous = node
        neighbor = None
        i = 1
        while True:
            if isNode(actual, baseMap):
                neighbor = actual
                break
            
            valid = False
            for step in actual.adjacent():
                if not isWalkable(step, baseMap) or step == previous:
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
        
        neighborChar = baseMap[neighbor.x][neighbor.y]
        neighbors.get(identifier).update({neighborChar if isKey(neighborChar) or isDoor(neighborChar) else str(neighbor.x*100+neighbor.y):i})
        
G=nx.Graph()
G.add_nodes_from(neighbors.keys())
for (node, data) in neighbors.items():
    for (neighbor, distance) in data.items():
        G.add_edge(node, neighbor, length=distance)

toRemove = []
while (True):
    for node in toRemove:
        G.remove_node(node)
    
    toRemove = []

    for (node, degree) in G.degree:
        if degree != 1:
            continue
        
        if len(node) > 1:
            toRemove.append(node)
        elif isDoor(node):
            toRemove.append(node)
    
    if len(toRemove) == 0:
        break

toRemove = []

for (node, degree) in G.degree:
    if degree != 2:
        continue
    
    if len(node) > 1:
        toRemove.append(node)

for node in toRemove:
    disolve(node, G)

sequence = "ayorjdgtukbhcivpezwqxfsmln"
distance = 0
for next in sequence:
    distance += nx.shortest_path_length(G, position, next, weight='length')
    position = next

print(distance)


color_map = []
for node in G:
    if len(node) > 1:
        color_map.append('grey')
    else: color_map.append('green' if isKey(node) else 'red')

pos = nx.kamada_kawai_layout(G)
nx.draw(G, pos=pos, node_color = color_map, with_labels=True)
plt.savefig("simple_path.png") # save as png
plt.show() # display
