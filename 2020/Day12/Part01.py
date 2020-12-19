from math import cos, radians, sin
import re

def parse(line: str):
    match = re.search(r'([A-Z])([0-9]*)', line)
    return (match.group(1), int(match.group(2)))

lines = [parse(line) for line in open("Day12/input", "r").read().strip().split("\n")]

pos = (0, 0)
direction = 0
for instr, amount in lines:
    if instr in 'NS':
        pos = (pos[0], pos[1] + amount * (-1 if instr == 'N' else 1))
        continue
    if instr in 'EW':
        pos = (pos[0] + amount * (-1 if instr == 'W' else 1), pos[1])
        continue
    if instr == 'F':
        pos = (pos[0] + cos(radians(direction)) * amount, pos[1] - sin(radians(direction)) * amount)
        continue
    if instr in 'LR':
        direction += amount * (-1 if instr == 'R' else 1)
        continue

print(int(round(abs(pos[0]) + abs(pos[1]))))
