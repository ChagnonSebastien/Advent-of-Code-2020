initialInput = 2

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
                
                if instruction[0] == 1:
                    self.data[parameters[2][0] + (self.relativeBase if parameters[2][1] == 2 else 0)] = getValue(parameters[0], self.data, self.relativeBase) + getValue(parameters[1], self.data, self.relativeBase)

                elif instruction[0] == 2:
                    self.data[parameters[2][0] + (self.relativeBase if parameters[2][1] == 2 else 0)] = getValue(parameters[0], self.data, self.relativeBase) * getValue(parameters[1], self.data, self.relativeBase)

                elif instruction[0] == 3:
                    self.data[parameters[0][0] + (self.relativeBase if parameters[0][1] == 2 else 0)] = self.inputValue

                elif instruction[0] == 4:
                    # remove the offset increment line if using print rather than return
                    self.i += parametersLength(instruction[0]) + 1
                    return getValue(parameters[0], self.data, self.relativeBase)

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
                    error("Done!")

                self.i += parametersLength(instruction[0]) + 1
            
            except IndexError:
                for x in range(len(self.data)):
                    self.data.append(0)

def turn(actual, direction):
    if actual[0] == 0 and actual[1] == 1:
        return (1 if direction == 0 else -1, 0)
    if actual[0] == 0 and actual[1] == -1:
        return (-1 if direction == 0 else 1, 0)
    if actual[0] == 1 and actual[1] == 0:
        return (0, -1 if direction == 0 else 1)
    if actual[0] == -1 and actual[1] == 0:
        return (0, 1 if direction == 0 else -1)

def onWhite(position, canvas):
    if canvas.get(position[0]) is None:
        return 0
    if canvas.get(position[0]).get(position[1]) is None:
        return 0
    return canvas.get(position[0]).get(position[1])

def paint(position, color, canvas):
    if canvas.get(position[0]) is None:
        canvas.update( {position[0]: dict()} )
    
    canvas.get(position[0]).update( {position[1]:color} )

def move(position, facing):
    return (position[0] + facing[0], position[1] + facing[1])
    

canvas = dict()
position = (0,0)
facing = (0,-1)

cpu = IntcodeCPU([int(x) for x in open("input.txt", "r").read().split(',')], initialInput)
try:
    while True:
        paintColor = cpu.send(onWhite(position, canvas))
        paint(position, paintColor, canvas)

        direction = cpu.run()
        facing = turn(facing, direction)
        position = move(position, facing)
except:
    n = 0
    for col in canvas.keys():
        n += len(canvas.get(col))
    print(n)