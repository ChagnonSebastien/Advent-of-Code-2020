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

class Amplifier:
    i = 0

    def __init__(self, phase):
        self.inputValue = phase
        self.data = [int(x) for x in open("input.txt", "r").read().split(',')]
        self.run()

    def send(self, inputValue):
        self.inputValue = inputValue
        return self.run()

    def run(self):

        while True:
            instruction = parseInstruction(self.data[self.i])
            parameters = [(self.data[self.i + index + 1], mode) for index,mode in enumerate(instruction[1])]
            
            if instruction[0] == 1:
                self.data[parameters[2][0]] = getValue(parameters[0], self.data) + getValue(parameters[1], self.data)

            elif instruction[0] == 2:
                self.data[parameters[2][0]] = getValue(parameters[0], self.data) * getValue(parameters[1], self.data)

            elif instruction[0] == 3:
                if self.inputValue is None:
                    break
                
                self.data[parameters[0][0]] = self.inputValue
                self.inputValue = None

            elif instruction[0] == 4:
                self.i += parametersLength(instruction[0]) + 1
                return getValue(parameters[0], self.data)

            elif instruction[0] == 5:
                if getValue(parameters[0], self.data) != 0:
                    self.i = getValue(parameters[1], self.data)
                    continue
                
            elif instruction[0] == 6:
                if getValue(parameters[0], self.data) == 0:
                    self.i = getValue(parameters[1], self.data)
                    continue
                
            elif instruction[0] == 7:
                self.data[parameters[2][0]] = 1 if getValue(parameters[0], self.data) < getValue(parameters[1], self.data) else 0
                
            elif instruction[0] == 8:
                self.data[parameters[2][0]] = 1 if getValue(parameters[0], self.data) == getValue(parameters[1], self.data) else 0

            else:
                error("Amplifier stopped")

            self.i += parametersLength(instruction[0]) + 1

maximum = 0
for phases in list(permutations([5,6,7,8,9])):
    amplifiers = [Amplifier(phase) for phase in phases]

    thrusterOutput = initialInput
    try:
        while True:
            currentValue = thrusterOutput
            for amplifier in amplifiers:
                currentValue = amplifier.send(currentValue)
            thrusterOutput = currentValue
    except:
        if thrusterOutput > maximum:
            maximum = thrusterOutput

print(maximum)