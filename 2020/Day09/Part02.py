from collections import deque
from itertools import combinations

lines = [int(line) for line in open("Day09/input", "r").read().strip().split("\n")]
answer = 0
memory = deque(lines[:25])

for line in lines[25:]:
    if not any([a + b == line for a, b in combinations(memory, 2)]):
        answer = line
        break

    memory.append(line)
    memory.popleft()

i = 0
memory = deque()
while sum(memory) != answer:
    memory.append(lines[i])
    while sum(memory) > answer:
        memory.popleft()
    i+=1

print(min(memory) + max(memory))