from math import floor
import numpy as np
import sys
np.set_printoptions(threshold=sys.maxsize)

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
        
class InputExpected(Exception):

    def __init__(self, data):
        self.data = data
   
class ProgramEnded(Exception):
    pass

class IntcodeCPU:
    i = 0
    relativeBase = 0

    inputValue = None

    def __init__(self, data):
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
                    if self.inputValue is None:
                        #raise InputExpected
                        break

                    self.data[parameters[0][0] + (self.relativeBase if parameters[0][1] == 2 else 0)] = self.inputValue
                    self.inputValue = None

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
                    raise ProgramEnded

                self.i += parametersLength(instruction[0]) + 1
            
            except IndexError:
                for x in range(len(self.data)):
                    self.data.append(0)

def isOnMap(coordinate, map):
    if coordinate[0] < 0 or coordinate[1] < 0:
        return False
    if coordinate[0] >= len(map) or coordinate[1] >= len(map[coordinate[0]]):
        return False
    return True

def turn(position, facing, map):
    adjacentPaths = 0
    for direction in [(1,0), (-1,0), (0,1), (0,-1)]:
        if direction[0] + facing[0] == 0 and direction[1] + facing[1] == 0:
            continue

        adjacentPoint = (position[0] + direction[0], position[1] + direction[1])
        if not isOnMap(adjacentPoint, map):
            continue
        
        if map[adjacentPoint[0]][adjacentPoint[1]] != ".":
            return (turnAscii(facing, direction), direction)
    raise Exception

def turnAscii(comingFrom, goingTo):
    if comingFrom[0] == 0:
        return 82 if (comingFrom[1] == 1) == (goingTo[0] < 0) else 76
    else:
        return 76 if (comingFrom[0] == 1) == (goingTo[1] < 0) else 82

def facingDirection(char):
    if char == 60:
        return (-1, 0)
    elif char == 62:
        return (1, 0)
    elif char == 94:
        return (0, -1)
    else:
        return (0, 1)

def advance(position, facing, map):
    distance = 0
    while True:
        newPosition = (position[0] + facing[0], position[1] + facing[1])
        if not isOnMap(newPosition, map):
            return (distance, position)

        if map[newPosition[0]][newPosition[1]] == ".":
            if distance == 0:
                raise Exception
            return (distance, position)
        
        distance += 1
        position = newPosition


cpu = IntcodeCPU([2 if i == 0 else int(x) for i,x in enumerate(open("input.txt", "r").read().split(','))])
output = ""
facing = None
positionIndex = 0

try:
    last  = -1
    while True:
        pixel = cpu.run()

        if pixel == 10 and last == 10:
            raise ProgramEnded

        if pixel != 35 and pixel != 46 and pixel != 10:
            facing = facingDirection(pixel)
            positionIndex = len(output.replace("\n", ""))

        output = output + str(chr(pixel))
        last = pixel

except ProgramEnded:
    pass

map = np.array([[c for c in line] for line in output.rsplit()]).transpose()
position = (positionIndex % len(map), floor(positionIndex / len(map)))

commands = []
try:
    while True:
        (direction, facing) = turn(position, facing, map)
        commands.append(direction)
        (distance, position) = advance(position, facing, map)
        for char in str(distance):
            commands.append(48 + int(char))
except Exception as e:
    pass

class NotFound(Exception):
    pass

def compress(commands, maxLength):
    for a in range(1, maxLength + 1):
        for b in range(1, maxLength + 1):
            for c in range(1, maxLength + 1):
                lengths = [a, b, c]
                sequence = []
                functions = []

                try:
                    i = 0
                    while i < len(commands):
                        matched = False
                        for f in range(len(functions)):
                            if functions[f] == commands[i:i+len(functions[f])]:
                                sequence.append(65 + f)
                                i += len(functions[f])
                                matched = True
                        
                        if matched:
                            continue

                        if len(functions) == 3:
                            raise NotFound

                        functions.append(commands[i:i+lengths[len(functions)]])

                    if len(sequence) <= maxLength:
                        return (sequence, functions)

                except NotFound:
                    continue

    raise NotFound

def parse(message):
    stream = []
    for i in range(len(message)):
        stream.append(message[i])
        if i < len(message) - 1:
            if not (message[i] >= 48 and message[i] < 58 and message[i+1] >= 48 and message[i+1] < 58):
                stream.append(44)

    return stream + [10]

(sequence, functions) = compress(commands, 10)

sending = [parse(sequence)] + [parse(f) for f in functions] + [[110, 10]]

for stream in sending:
    while True:
        answer = cpu.run()
        if answer is None:
            break

    for m in stream:
        answer = cpu.send(m)

dust = 0
try:
    while True:
        dust = cpu.run()
except ProgramEnded:
    pass

print(dust)