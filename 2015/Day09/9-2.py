import networkx as nx
import numpy as np
from itertools import permutations

paths = [line.split() for line in open("input.txt", "r")]

G = nx.Graph()

for path in paths:
    a = path[0]

    b = path[2]
    
    d = int(path[4])
    G.add_edge(a, b, weight=d)

routes = permutations(G.nodes())
longestRouteDistance = 0

for route in routes:
    distance = 0
    for i in range(1, len(route)):
        distance += G[route[i-1]][route[i]]['weight']
    
    if distance > longestRouteDistance:
        longestRouteDistance = distance

print(longestRouteDistance)