from math import prod
import re

def parseRule(line: str):
    match = re.search(r'([a-z ]*): ([0-9]*)-([0-9]*) or ([0-9]*)-([0-9]*)', line)
    return (match.group(1), int(match.group(2)), int(match.group(3)), int(match.group(4)), int(match.group(5)))

def parseTicket(line: str):
    return [int(i) for i in line.split(',')]

lines = [line for line in open("Day16/input", "r").read().strip().split("\n")]
splits = [i for i, n in enumerate(lines) if n == ""]
rules = [parseRule(line) for line in lines[:splits[0]]]
myTicket = parseTicket(lines[splits[0] + 2])
otherTickets = [parseTicket(line) for line in lines[splits[1] + 2:]]

def valid(ticket):
    return all(any((i >= a and i <= b) or (i >= c and i <= d) for _, a, b, c, d in rules) for i in ticket)

validTickets = [ticket for ticket in otherTickets if valid(ticket)]
possibilities = [[rule[0] for rule in rules] for _ in range(0, len(myTicket))]

for ticket in validTickets:
    for i, v in enumerate(ticket):
        validFields = [text for text, a, b, c, d in rules if ((v >= a and v <= b) or (v >= c and v <= d))]
        possibilities[i] = [text for text in possibilities[i] if text in validFields]

while any([len(p) > 1 for p in possibilities]):
    for i, p in enumerate(possibilities):
        if len(p) == 1:
            for j, r in enumerate(possibilities):
                if p[0] in r and i != j:
                    possibilities[j] = [s for s in possibilities[j] if s != p[0]]

departures = [i for i, p in enumerate(possibilities) if 'departure' in p[0]]
print(prod([v for i, v in enumerate(myTicket) if i in departures]))
