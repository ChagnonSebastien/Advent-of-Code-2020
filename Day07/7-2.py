lines = [line.split() for line in open("input.txt", "r")]

instructions = []
values = dict()

for line in lines:
    if line[2] == "b":
        values.update({"b": 3176})
    elif len(line) == 3:
        try:
            values.update({line[2]: int(line[0])})
        except:
            instructions.append(("ASSING", [line[0]], line[2]))
    elif len(line) == 4:
        a = line[1]
        try:
            a = int(a)
        except:
            pass
        instructions.append(("NOT", [a], line[3]))
    else:
        a = line[0]
        try:
            a = int(a)
        except:
            pass
        b = line[2]
        try:
            b = int(b)
        except:
            pass
        instructions.append((line[1], [a, b], line[4]))

while values.get("a") is None:
    for instruction in instructions:
        if values.get(instruction[2]) is not None:
            continue

        if not all([True if isinstance(i, int) else values.get(i) is not None for i in instruction[1]]):
            continue

        inputs = [i if isinstance(i, int) else values.get(i) for i in instruction[1]]

        if instruction[0] == "ASSING":
            values.update({instruction[2]: inputs[0]})
        elif instruction[0] == "NOT":
            values.update({instruction[2]: ~ inputs[0]})
        elif instruction[0] == "LSHIFT":
            values.update({instruction[2]: inputs[0] << inputs[1]})
        elif instruction[0] == "RSHIFT":
            values.update({instruction[2]: inputs[0] >> inputs[1]})
        elif instruction[0] == "AND":
            values.update({instruction[2]: inputs[0] & inputs[1]})
        elif instruction[0] == "OR":
            values.update({instruction[2]: inputs[0] | inputs[1]})
        else:
            raise Exception("Unknown Command")

print(values.get("a"))
