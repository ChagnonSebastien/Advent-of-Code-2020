from itertools import combinations
from math import prod

weights = set([int(line) for line in open("Day24/input", "r").read().strip().split("\n")])
totalWeight = sum(weights)

def isDisivible(currentWeight, remainder):
    if currentWeight == sum(remainder):
        return True

    if currentWeight > sum(remainder):
        return False
    
    for element in remainder:
        if isDisivible(currentWeight + element, [w for w in remainder if w != element]):
            return True
    
    return False
        

configurations = set()
i = 0


while len(configurations) == 0:
    i += 1
    for legConfiguration in combinations(weights, i):
        if sum(legConfiguration) != totalWeight / 3:
            continue

        if isDisivible(0, [w for w in weights if w not in legConfiguration]):
            configurations.add(legConfiguration)

print(min([prod(c) for c in configurations]))
