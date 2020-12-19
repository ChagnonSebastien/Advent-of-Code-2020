import regex as re
lines = [line for line in open("Day19/input.txt", "r").read().strip().split("\n")]

instructions = [(line.split(r' => ')[0], line.split(r' => ')[1]) for line in lines if " => " in line]
goal = lines.pop()
steps = 0

while goal != "e":
    for i in instructions:
        if i[1] in goal:
            goal = goal.replace(i[1], i[0], 1)
            steps += 1

print(steps)

