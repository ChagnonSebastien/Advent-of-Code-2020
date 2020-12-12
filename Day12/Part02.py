from math import cos, radians, sin
import re

def parse(line: str):
    match = re.search(r'([A-Z])([0-9]*)', line)
    return (match.group(1), int(match.group(2)))

lines = [parse(line) for line in open("Day12/input", "r").read().strip().split("\n")]

pos = (0, 0)
wp = (10, -1)
for instr, amount in lines:
    if instr in 'NS':
        wp = (wp[0], wp[1] + amount * (-1 if instr == 'N' else 1))
        continue
    if instr in 'EW':
        wp = (wp[0] + amount * (-1 if instr == 'W' else 1), wp[1])
        continue
    if instr == 'F':
        pos = (pos[0] + wp[0] * amount, pos[1] + wp[1]  * amount)
        continue
    if instr in 'LR':
        angle = radians(amount * (-1 if instr == 'L' else 1))
        wp = (cos(angle) * wp[0] - sin(angle) * wp[1], sin(angle) * wp[0] + cos(angle) * wp[1])
        continue

print(int(round(abs(pos[0]) + abs(pos[1]))))
