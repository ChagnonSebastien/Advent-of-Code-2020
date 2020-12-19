import re

def parse(line: str):
    return (re.search(r'^([a-z]+ [a-z]+) bags', line).group(1), re.findall(r'([0-9]+) ([a-z]+ [a-z]+) bag', line))

lines = [parse(line) for line in open("Day07/input", "r").read().strip().split("\n")]
rules = {color: {c2: int(amount) for amount, c2 in contents} for color, contents in lines}

def isValid(c):
    return any(color == 'shiny gold' or isValid(color) for color in rules.get(c).keys())

print(len([color for color in rules.keys() if isValid(color)]))
