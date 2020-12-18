import re

lines = [line.replace(' ', '') for line in open("Day18/input", "r").read().strip().split("\n")]

def evaluate(line: str):
    while not re.match(r'^[0-9]*$', line):
        match = re.search(r'\(([^\(]+?)\)', line)
        if match:
            line = line.replace(match.group(), str(evaluate(match.group(1))), 1)
            continue

        match = re.search(r'([0-9]+)\+([0-9]+)', line)
        if match:
            line = line.replace(match.group(), str(int(match.group(1)) + int(match.group(2))), 1)
            continue

        match = re.search(r'([0-9]+)\*([0-9]+)', line)
        if match:
            line = line.replace(match.group(), str(int(match.group(1)) * int(match.group(2))), 1)
            continue
    return int(line)
 
print(sum([evaluate(line) for line in lines]))
