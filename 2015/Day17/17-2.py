import numpy as np

containers = [int(line) for line in open("input.txt", "r")]

def testCombinations(layer, remainder):
    if layer >= len(containers):
        return []

    remainderIncludingLayer = remainder - containers[layer]
    if remainderIncludingLayer == 0:
        return [[containers[layer]]] + testCombinations(layer + 1, remainder)
    elif remainderIncludingLayer < 0:
        return testCombinations(layer + 1, remainder)
    else:
        solutuionsIncludingLayer = testCombinations(layer + 1, remainderIncludingLayer)
        for solution in solutuionsIncludingLayer:
            solution.append(containers[layer])
        return solutuionsIncludingLayer + testCombinations(layer + 1, remainder)

possibilities = testCombinations(0, 150)
possibleLengths = np.array([len(possibility) for possibility in possibilities])

minimumContainers = np.min(possibleLengths)
print(np.sum(possibleLengths == minimumContainers))