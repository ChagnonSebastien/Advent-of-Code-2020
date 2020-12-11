import numpy as np

lines = [line for line in open("Day11/input", "r").read().strip().split("\n")]
occurences = 0
value = 0

while np.sum(np.isin(lines, "#")) != value or occurences < 5:
    v = np.sum(np.isin(lines, "#"))
    if np.sum(np.isin(lines, "#")) == value:
        occurences += 1
    else:
        value = np.sum(np.isin(lines, "#"))
        occurences = 1

    new = []
    for i in range(0, len(lines)):
        newLine = []
        for j in range(0, len(lines[0])):
            if (lines[i][j] == '.'):
                newLine.append(".")
                continue

            amount = 0
            for x, y in [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]:
                ray = (i + x, j + y)
                while not (ray[0] < 0 or ray[0] >= len(lines) or ray[1] < 0 or ray[1] >= len(lines[0])):
                    if lines[ray[0]][ray[1]] != ".":
                        amount += lines[ray[0]][ray[1]] == "#"
                        break
                    ray = (ray[0] + x, ray[1] + y)
            
            newLine.append("#" if amount == 0 else ("L" if amount >= 5 else lines[i][j]))

        new.append(newLine)

    lines = new

print(value)