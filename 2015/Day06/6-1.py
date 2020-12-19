import re
import numpy as np

instructions = [[[int(n) for n in section.strip().split(',')] if ',' in section else section.strip() for section in re.split('([0-9]+,[0-9]+)', instruction.strip())] for instruction in open("input.txt", "r")]

canvas = np.zeros((1000, 1000), bool)

for instruction in instructions:
    if "turn" in instruction[0]:
        goal = "on" in instruction[0]
        for i in range(instruction[1][0], instruction[3][0] + 1):
            for j in range(instruction[1][1], instruction[3][1] + 1):
                canvas[i][j] = goal

    else:
        for i in range(instruction[1][0], instruction[3][0] + 1):
            for j in range(instruction[1][1], instruction[3][1] + 1):
                canvas[i][j] = not canvas[i][j]


print(np.sum(canvas))