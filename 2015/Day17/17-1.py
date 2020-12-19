containers = [int(line) for line in open("input.txt", "r")]

def testCombinations(layer, remainder):
    if layer >= len(containers):
        return 0

    remainderIncludingLayer = remainder - containers[layer]
    if remainderIncludingLayer == 0:
        return 1 + testCombinations(layer + 1, remainder)
    elif remainderIncludingLayer < 0:
        return testCombinations(layer + 1, remainder)
    else:
        return testCombinations(layer + 1, remainder) + testCombinations(layer + 1, remainderIncludingLayer)

print(testCombinations(0, 150))

