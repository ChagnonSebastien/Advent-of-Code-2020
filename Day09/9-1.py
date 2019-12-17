initialInput = 1

def parametersLength(op):
    if op == 1 or op == 2 or op == 7 or op == 8:
        return 3
    if op == 3 or op == 4 or op == 9:
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

def getValue(parameter, data, relativeBase):
    if parameter[1] == 0:
        return data[parameter[0]]
    elif parameter[1] == 1:
        return parameter[0]
    else:
        return data[relativeBase + parameter[0]]
        

class IntcodeCPU:
    i = 0
    relativeBase = 0

    def __init__(self, data, inputValue):
        self.inputValue = inputValue
        self.data = data

    def send(self, inputValue):
        self.inputValue = inputValue
        return self.run()

    def run(self):

        while True:
            try:
                instruction = parseInstruction(self.data[self.i])
                parameters = [(self.data[self.i + index + 1], mode) for index,mode in enumerate(instruction[1])]
                #print(instruction)
                #print(parameters)
                
                if instruction[0] == 1:
                    self.data[parameters[2][0] + (self.relativeBase if parameters[2][1] == 2 else 0)] = getValue(parameters[0], self.data, self.relativeBase) + getValue(parameters[1], self.data, self.relativeBase)

                elif instruction[0] == 2:
                    self.data[parameters[2][0] + (self.relativeBase if parameters[2][1] == 2 else 0)] = getValue(parameters[0], self.data, self.relativeBase) * getValue(parameters[1], self.data, self.relativeBase)

                elif instruction[0] == 3:
                    if self.inputValue is None:
                        break
                    
                    self.data[parameters[0][0] + (self.relativeBase if parameters[0][1] == 2 else 0)] = self.inputValue
                    self.inputValue = None

                elif instruction[0] == 4:
                    print(getValue(parameters[0], self.data, self.relativeBase))

                elif instruction[0] == 5:
                    if getValue(parameters[0], self.data, self.relativeBase) != 0:
                        self.i = getValue(parameters[1], self.data, self.relativeBase)
                        continue
                    
                elif instruction[0] == 6:
                    if getValue(parameters[0], self.data, self.relativeBase) == 0:
                        self.i = getValue(parameters[1], self.data, self.relativeBase)
                        continue
                    
                elif instruction[0] == 7:
                    self.data[parameters[2][0] + (self.relativeBase if parameters[2][1] == 2 else 0)] = 1 if getValue(parameters[0], self.data, self.relativeBase) < getValue(parameters[1], self.data, self.relativeBase) else 0
                    
                elif instruction[0] == 8:
                    self.data[parameters[2][0] + (self.relativeBase if parameters[2][1] == 2 else 0)] = 1 if getValue(parameters[0], self.data, self.relativeBase) == getValue(parameters[1], self.data, self.relativeBase) else 0

                elif instruction[0] == 9:
                    self.relativeBase += getValue(parameters[0], self.data, self.relativeBase)

                else:
                    break

                #print(self.data)
                #print(parametersLength(instruction[0]))
                self.i += parametersLength(instruction[0]) + 1
                #print("i: " + str(self.i))
            
            except IndexError:
                #print("---")
                #print(len(self.data))
                for x in range(len(self.data)):
                    self.data.append(0)
                #print(len(self.data))

cpu = IntcodeCPU([int(x) for x in open("input.txt", "r").read().split(',')], initialInput)
cpu.run()