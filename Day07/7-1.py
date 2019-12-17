from itertools import permutations

initialInput = 0

def parametersLength(op):
    if op == 1 or op == 2 or op == 7 or op == 8:
        return 3
    if op == 3 or op == 4:
        return 1
    if op == 5 or op == 6:
        return 2

    return 0

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

def run(phase, inputValue):

    data = [int(x) for x in open("input.txt", "r").read().split(',')]

    phaseGiven = False
    i = 0

    while True:
        instruction = parseInstruction(data[i])
        parameters = [(data[i + index + 1], mode) for index,mode in enumerate(instruction[1])]
        
        if instruction[0] == 1:
            data[parameters[2][0]] = getValue(parameters[0], data) + getValue(parameters[1], data)

        elif instruction[0] == 2:
            data[parameters[2][0]] = getValue(parameters[0], data) * getValue(parameters[1], data)

        elif instruction[0] == 3:
            data[parameters[0][0]] = inputValue if phaseGiven else phase
            phaseGiven = True

        elif instruction[0] == 4:
            return getValue(parameters[0], data)

        elif instruction[0] == 5:
            if getValue(parameters[0], data) != 0:
                i = getValue(parameters[1], data)
                continue
            
        elif instruction[0] == 6:
            if getValue(parameters[0], data) == 0:
                i = getValue(parameters[1], data)
                continue
            
        elif instruction[0] == 7:
            data[parameters[2][0]] = 1 if getValue(parameters[0], data) < getValue(parameters[1], data) else 0
            
        elif instruction[0] == 8:
            data[parameters[2][0]] = 1 if getValue(parameters[0], data) == getValue(parameters[1], data) else 0

        else:
            break

        i += parametersLength(instruction[0]) + 1


maximum = 0
for phases in list(permutations([0,1,2,3,4])):
    currentValue = initialInput
    for phase in phases:
        currentValue = run(phase, currentValue)

    if currentValue > maximum:
        maximum = currentValue

print(maximum)