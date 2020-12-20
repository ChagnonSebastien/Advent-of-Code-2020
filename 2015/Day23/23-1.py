import re

def parse(instruction):
    return re.match(r'([a-z]+) (a|b|(?:\+|\-)[0-9]+)(?:, ((?:\+|\-)[0-9]+))?', instruction).groups()

lines = [parse(line) for line in open("Day23/input", "r").read().strip().split("\n")]
registers = dict()

offset = 0
while offset < len(lines):
    instruction, x, y = lines[offset]
    jump = 1
    if instruction == 'hlf':
        registers.update({ x: registers.get(x, 0) / 2 })
    elif instruction == 'tpl':
        registers.update({ x: registers.get(x, 0) * 3 })
    elif instruction == 'inc':
        registers.update({ x: registers.get(x, 0) + 1 })
    elif instruction == 'jmp':
        jump = int(x)
    elif instruction == 'jie' and registers.get(x, 0) % 2 == 0:
        jump = int(y)
    elif instruction == 'jio' and registers.get(x, 0) == 1:
        jump = int(y)
    offset += jump

print(registers.get('b', 0))