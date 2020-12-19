import networkx as nx
import numpy as np
from itertools import permutations

paths = [line.rstrip(".\n").split() for line in open("input.txt", "r")]
G = nx.Graph()

for path in paths:
    a = path[0]
    b = path[10]
    
    d = int(path[3]) * (1 if path[2] == "gain" else -1)

    try:
        G[a][b]['weight'] += d
    except:
        G.add_edge(a, b, weight=d)

routes = permutations(G.nodes())
maximumHappiness = 0

for route in routes:
    hapiness = 0
    for i in range(0, len(route)):
        hapiness += G[route[i]][route[(i + 1) % len(G.nodes())]]['weight']
    
    if hapiness > maximumHappiness:
        maximumHappiness = hapiness

print(maximumHappiness)