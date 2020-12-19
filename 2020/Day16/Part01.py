import re

def parseRule(line: str):
    match = re.search(r'([a-z ]*): ([0-9]*)-([0-9]*) or ([0-9]*)-([0-9]*)', line)
    return (match.group(1), int(match.group(2)), int(match.group(3)), int(match.group(4)), int(match.group(5)))

def parseTicket(line: str):
    return [int(i) for i in line.split(',')]

lines = [line for line in open("Day16/input", "r").read().strip().split("\n")]
splits = [i for i, n in enumerate(lines) if n == ""]
rules = [parseRule(line) for line in lines[:splits[0]]]
otherTickets = [parseTicket(line) for line in lines[splits[1] + 2:]]

def errorRate(ticket):
    return sum([i for i in ticket if not any((i >= a and i <= b) or (i >= c and i <= d) for _, a, b, c, d in rules)])

print(sum([errorRate(ticket) for ticket in otherTickets]))
