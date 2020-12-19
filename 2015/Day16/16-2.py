import re

sues = [{attributes.split()[0].rstrip(':'): int(attributes.split()[1]) for attributes in re.split('^Sue [0-9]+: ', line.rstrip())[1].split(', ')} for line in open("input.txt", "r")]

gifter = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1
}

for (index, sue) in enumerate(sues):
    foundSue = True
    for label in sue.keys():
        if label == "cats" or label == "trees":
            if sue.get(label) <= gifter.get(label):
                foundSue = False
                break
        elif label == "pomeranians" or label == "goldfish":
            if sue.get(label) >= gifter.get(label):
                foundSue = False
                break
        elif sue.get(label) != gifter.get(label):
            foundSue = False
            break
    
    if foundSue:
        print("Gifter is Sue ", index + 1)