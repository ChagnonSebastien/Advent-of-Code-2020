import re
import numpy as np

instructions = [[[int(n) for n in section.strip().split(',')] if ',' in section else section.strip() for section in re.split('([0-9]+,[0-9]+)', instruction.strip())] for instruction in open("input.txt", "r")]

canvas = np.zeros((1000, 1000), int)
print(canvas)

for instruction in instructions:
    if instruction[0] == "turn off":
        for i in range(instruction[1][0], instruction[3][0] + 1):
            for j in range(instruction[1][1], instruction[3][1] + 1):
                canvas[i][j] = max([0, canvas[i][j] - 1])

    else:
        amount = 2 if instruction[0] == "toggle" else 1
        for i in range(instruction[1][0], instruction[3][0] + 1):
            for j in range(instruction[1][1], instruction[3][1] + 1):
                canvas[i][j] = canvas[i][j] + amount


print(np.sum(canvas))