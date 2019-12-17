manualInput = 1

def parametersLength(op):
    if op == 99:
        return 0

    return 3 if op <= 2 else 1

def parseInstruction(instruction):
    stringified = str(instruction)
    op = int(stringified[len(stringified)-2:])
    modes = [int(i) for i in stringified[:len(stringified)-2]]
    modes.reverse()
    while len(modes) < parametersLength(op):
        modes.append(0)

    return (op,modes)

def getValue(parameter, data):
    return data[parameter[0]] if parameter[1] == 0 else parameter[0]

data = [int(x) for x in open("input.txt", "r").read().split(',')]

i = 0
while True:
    instruction = parseInstruction(data[i])
    parameters = [(data[i + index + 1], mode) for index,mode in enumerate(instruction[1])]
    
    if instruction[0] == 1:
        data[parameters[2][0]] = getValue(parameters[0], data) + getValue(parameters[1], data)
    elif instruction[0] == 2:
        data[parameters[2][0]] = getValue(parameters[0], data) * getValue(parameters[1], data)
    elif instruction[0] == 3:
        data[parameters[0][0]] = manualInput
    elif instruction[0] == 4:
        print(getValue(parameters[0], data))
    else:
        break

    i += parametersLength(instruction[0]) + 1