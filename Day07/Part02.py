import re

def parse(line: str):
    return (re.search(r'^([a-z]+ [a-z]+) bags', line).group(1), re.findall(r'([0-9]+) ([a-z]+ [a-z]+) bag', line))

lines = [parse(line) for line in open("Day07/input", "r").read().strip().split("\n")]
rules = {color: {c2: int(amount) for amount, c2 in contents} for color, contents in lines}

def getAmount(c):
    return sum([amount + amount * getAmount(c2) for c2, amount in rules.get(c, {}).items()])

print(getAmount('shiny gold'))
