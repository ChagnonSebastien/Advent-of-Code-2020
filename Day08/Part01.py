import re

def parse(line: str):
    match = re.search(r'([a-z]+) ([+-][0-9]+)', line)
    return (match.group(1), int(match.group(2)))

lines = [parse(line) for line in open("Day08/input", "r").read().strip().split("\n")]

executed = []
acc = 0
offset = 0
while offset < len(lines):
    if offset in executed:
        print(acc)
        break

    executed.append(offset)
    instr, number = lines[offset]
    if instr == 'acc':
        acc += number
    if instr == 'jmp':
        offset += number
    else:
        offset += 1
