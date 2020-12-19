import regex as re
lines = [line for line in open("Day19/input.txt", "r").read().strip().split("\n")]

instructions = [(line.split(r' => ')[0], line.split(r' => ')[1]) for line in lines if " => " in line]
molecule = [a[0] for a in re.findall(r'(([A-Z][a-z]?)|e)', lines.pop())]

uniques = []
for i, m in enumerate(molecule):
    for inst in instructions:
        if m == inst[0]:
            new = "".join([m if i != j else inst[1] for j, m in enumerate(molecule)])
            if new not in uniques:
                uniques.append(new)

print(len(uniques))



