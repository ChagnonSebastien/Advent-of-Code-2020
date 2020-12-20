from itertools import combinations
from math import prod

weights = [int(line) for line in open("Day24/input", "r").read().strip().split("\n")]
weights.reverse()
totalWeight = sum(weights)

def isDisivible(current, remainder):
    if len(remainder) == 0:
        return True
    
    r = remainder[0]
    if current[0] + r <= totalWeight / 4:
        if isDisivible((current[0] + r, current[1], current[2]), remainder[1:]):
            return True
    if current[1] + r <= totalWeight / 4:
        if isDisivible((current[0], current[1] + r, current[2]), remainder[1:]):
            return True
    if current[2] + r <= totalWeight / 4:
        if isDisivible((current[0], current[1], current[2] + r), remainder[1:]):
            return True
    
    return False

configurations = set()
i = 0

while len(configurations) == 0:
    i += 1
    for legConfiguration in combinations(weights, i):
        if sum(legConfiguration) != totalWeight / 4:
            continue

        if isDisivible((0, 0, 0), [w for w in weights if w not in legConfiguration]):
            configurations.add(legConfiguration)

print(min([prod(c) for c in configurations]))
