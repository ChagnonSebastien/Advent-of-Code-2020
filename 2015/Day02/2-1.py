dimentions = [[int(d) for d in line.rstrip().split('x')] for line in open("input.txt", "r").readlines()]

totalWrappingPaperNeeded = 0

for gift in dimentions:
    a = gift[0] * gift[1]
    b = gift[1] * gift[2]
    c = gift[0] * gift[2]
    smallest = min([a, b, c])
    totalWrappingPaperNeeded += (2* (a + b + c) + smallest)

print(totalWrappingPaperNeeded)