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
        rawValue = bin(int(value))[2:].zfill(36)
        finalValue = "".join([rawValue[i] if b == 'X' else b for i, b in enumerate(mask)])
        memory.update({ adress: int(finalValue, 2) })

print(sum(memory.values()))
