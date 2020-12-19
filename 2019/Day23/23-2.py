from math import floor
import numpy as np
import sys
from collections import deque
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
    pass
   
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
                        raise InputExpected
                        #break

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

program = [int(x) for i,x in enumerate(open("input.txt", "r").read().split(','))]


cpus = [IntcodeCPU(program.copy()) for _ in range(50)]
network = [deque([]) for _ in range(len(cpus))]

nat = (None, None)
idle = [0 for _ in range(len(cpus))]
lastSent = 0

for i in range(len(cpus)):
    try:
        cpus[i].send(i)
        while True:
            destination = cpus[i].run()
            while True:
                x = cpus[i].run()
                y = cpus[i].run()
                network[destination].append((x, y))
                destination = cpus[i].run()
    except InputExpected:
        pass

while True:
    minIdle = 50
    for time in idle:
        if time < minIdle:
            minIdle = time
    if minIdle == 50:
        network[0].append(nat)
        idle = [0 for _ in range(len(cpus))]
    
        if nat[1] == lastSent:
            print(nat[1])
            raise Exception
        lastSent = nat[1]

    for i in range(len(cpus)):
        activity = False
        try:
            if (len(network[i]) == 0):
                cpus[i].send(-1)
                destination = cpus[i].run()
                activity = True
                while True:
                    x = cpus[i].run()
                    y = cpus[i].run()
                    if destination == 255:
                        nat = (x, y)
                    else:
                        network[destination].append((x, y))
                    destination = cpus[i].run()
            else:
                activity = True
                (x, y) = network[i].popleft()
                cpus[i].send(x)
                try:
                    rer = cpus[i].run()
                    print(rer)
                except InputExpected:
                    pass
                cpus[i].send(y)
                destination = cpus[i].run()
                while True:
                    x = cpus[i].run()
                    y = cpus[i].run()
                    if destination == 255:
                        nat = (x, y)
                    else:
                        network[destination].append((x, y))
                    destination = cpus[i].run()
        except InputExpected:
            idle[i] = 0 if activity else idle[i]+1