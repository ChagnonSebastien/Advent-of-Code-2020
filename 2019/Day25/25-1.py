from math import floor
from collections import deque

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
cpu = IntcodeCPU(program)

setup = deque([
    "west",
    "south",
    "take pointer",
    "south",
    "take prime number",
    "west",
    "take coin",
    "east",
    "north",
    "north",
    "east",
    "south",
    "take festive hat",
    "north",
    "east",
    "south",
    "south",
    "take space heater",
    "south",
    "take astrolabe",
    "north",
    "north",
    "north",
    "north",
    "take wreath",
    "north",
    "west",
    "take dehydrated water",
    "north",
    "east"
])

objects = []

output = ""
try:
    while True:
        try:
            output = output + chr(cpu.run())
        except InputExpected:

            if len(setup) > 0:
                print(output)
                command = setup.popleft()
            else:
                command = input(output)

            output = ""
            
            if command.find("take") >= 0:
                objects.append(" ".join(command.split(" ")[1:]))

            if command == "solve":
                for j in range(len(objects)):
                    for c in ("drop " + objects[j]):
                        cpu.send(ord(c))
                        try:
                            while True:
                                cpu.run()
                        except InputExpected:
                            pass
                            
                    try:
                        cpu.send(10)
                        output = ""
                        while True:
                            output = output + chr(cpu.run())
                    except InputExpected:
                        print(output)

                output = ""
                for i in range(pow(2, len(objects))):
                    print("================== " + str(i))
                    for j in range(len(objects)):
                        if i & pow(2, j):
                            for c in ("take " + objects[j]):
                                cpu.send(ord(c))
                                try:
                                    while True:
                                        cpu.run()
                                except InputExpected:
                                    pass
                                    
                            output = ""
                            try:
                                cpu.send(10)
                                while True:
                                    output = output + chr(cpu.run())
                            except InputExpected:
                                print(output)
                    
                    for c in ("south"):
                        cpu.send(ord(c))
                        try:
                            while True:
                                cpu.run()
                        except InputExpected:
                            pass
                            
                    output = ""
                    try:
                        cpu.send(10)
                        while True:
                            output = output + chr(cpu.run())
                    except InputExpected:
                        print(output)
                    

                    if output.find("== Security Checkpoint ==") == -1:
                        break
                    else:
                        for j in range(len(objects)):
                            if i & pow(2, j):
                                for c in ("drop " + objects[j]):
                                    cpu.send(ord(c))
                                    try:
                                        while True:
                                            cpu.run()
                                    except InputExpected:
                                        pass
                                        
                                output = ""
                                try:
                                    cpu.send(10)
                                    while True:
                                        output = output + chr(cpu.run())
                                except InputExpected:
                                    print(output)

            else:
                for c in command:
                    cpu.send(ord(c))
                    try:
                        cpu.run()
                    except InputExpected:
                        pass
            cpu.send(10)

except ProgramEnded:
    pass

print(output)