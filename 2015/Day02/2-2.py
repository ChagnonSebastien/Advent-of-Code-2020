dimentions = [[int(d) for d in line.rstrip().split('x')] for line in open("input.txt", "r").readlines()]

totalRibbonNeeded = 0

for gift in dimentions:
    
    s = 2 * (gift[0] + gift[1] + gift[2] - max([gift[0], gift[1], gift[2]]))
    v = gift[0] * gift[1] * gift[2]

    totalRibbonNeeded += (s + v)

print(totalRibbonNeeded)