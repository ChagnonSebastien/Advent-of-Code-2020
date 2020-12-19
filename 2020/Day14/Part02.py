from itertools import combinations
import re

def parse(line: str):
    return re.search(r'(mask|mem\[([0-9]*)\]) = ([A-Z0-9]*)', line).group(1, 2, 3)

lines = [parse(line) for line in open("Day14/input", "r").read().strip().split("\n")]
memory = dict()
mask = None
for instr, adress, value in lines:
    if instr == 'mask':
        mask = value
    else:
        rawAddr = bin(int(adress))[2:].zfill(36)
        wildcardAddr = "".join([rawAddr[i] if b == '0' else '1' for i, b in enumerate(mask)])
        memory.update({ int(wildcardAddr, 2): int(value) })
        xPos = [i for i, b in enumerate(mask) if b == 'X']
        for i in range(1, len(xPos) + 1):
            for comb in combinations(xPos, i):
                memory.update({ int("".join(['0' if i in comb else b for i, b in enumerate(wildcardAddr)]), 2): int(value) })

print(sum(memory.values()))
