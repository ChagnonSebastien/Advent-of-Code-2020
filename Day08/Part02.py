import re

def parse(line: str):
    match = re.search(r'([a-z]+) ([+-][0-9]+)', line)
    return (match.group(1), int(match.group(2)))

lines = [parse(line) for line in open("Day08/input", "r").read().strip().split("\n")]
indexes = [(i, 'nop' if line[0] == 'jmp' else 'nop') for i, line in enumerate(lines) if line[0] == 'jmp' or line[0] == 'nop']
programs = [[(instr if i != a else new, number) for a, (instr, number) in enumerate(lines)] for i, new in indexes]

for program in programs:
    executed = []
    acc = 0
    offset = 0
    error = False
    while offset < len(program):
        if offset in executed:
            error = True
            break

        executed.append(offset)
        instr, number = program[offset]
        if instr == 'acc':
            acc += number
        if instr == 'jmp':
            offset += number
        else:
            offset += 1
    if error == False:
        print(acc)
        break
