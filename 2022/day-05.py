import re
import numpy as np

def parse_instructions(instruction):
    match = re.match(r'move ([0-9]+) from ([0-9]) to ([0-9])', instruction)
    return (int(match.group(1)), int(match.group(2)), int(match.group(3)))

input = open('input-05', 'r').read().rstrip()
[initial_state, raw_instructions] = input.split('\n\n')
grid = np.array([list(line) for line in initial_state.split('\n')[:-1]])
stacks = [list(np.flipud(grid[:, i*4+1][grid[:, i*4+1] != " "])) for i in range(int((grid.shape[1]+1)/4))]
instructions = [parse_instructions(instruction) for instruction in raw_instructions.split('\n')]

part1_stacks = [row[:] for row in stacks]
for (amount, _from, _to) in instructions:
    for _ in range(amount):
        crate = part1_stacks[_from-1].pop()
        part1_stacks[_to-1].append(crate)

print(f'Part1: {"".join([part1_stacks[-1:].pop() for part1_stacks in part1_stacks])}')

part2_stacks = [row[:] for row in stacks]
buffer = list()
for (amount, _from, _to) in instructions:
    for _ in range(amount):
        crate = part2_stacks[_from-1].pop()
        buffer.append(crate)
    for _ in range(amount):
        crate = buffer.pop()
        part2_stacks[_to-1].append(crate)


print(f'Part1: {"".join([part2_stacks[-1:].pop() for part2_stacks in part2_stacks])}')