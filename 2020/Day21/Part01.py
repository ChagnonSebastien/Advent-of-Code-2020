from functools import reduce
import re

def parse(line: str):
    match = re.search(r'([a-z ]*[a-z]+) \(contains (.*)\)', line)
    return (match.group(1).split(' '), match.group(2).split(', '))

receipes = [parse(line) for line in open("Day21/input", "r").read().strip().split("\n")]

allergensSet = set()
for _, allergens in receipes:
    for allergen in allergens:
        allergensSet.add(allergen)

associations = dict()
while len(associations) < len(allergensSet):
    for a in [a for a in allergensSet if a not in associations.keys()]:
        contains = [[i for i in ingredients if i not in associations.values()] for ingredients, allergens in receipes if a in allergens]
        count = dict()
        for ingredients in contains:
            for ingredient in ingredients:
                count.update({ ingredient: count.get(ingredient, 0) + 1 })
        potentials = [ingredient for ingredient, count in count.items() if count == len(contains)]
        if len(potentials) == 1:
            associations.update({ a: potentials[0] })

print(sum([sum([1 for i in ingredients if i not in associations.values()]) for ingredients, _ in receipes]))

allergens = list(associations.keys())
allergens.sort()
print(','.join([associations.get(allergen) for allergen in allergens]))
