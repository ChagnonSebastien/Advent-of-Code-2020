from random import randint
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

def displayMap(area, robot):
    minHeight = 0
    maxHeight = 0
    minWidth = 0
    maxWidth = 0
    for col in area.keys():
        if col > maxWidth:
            maxWidth = col
        if col < minWidth:
            minWidth = col

        for row in area.get(col):
            if row > maxHeight:
                maxHeight = row
            if row < minHeight:
                minHeight = row

    for i in range(minHeight, maxHeight + 1):
        row = ""
        for j in range(minWidth, maxWidth + 1):
            if robot[0] == j and robot[1] == i:
                row = row + "+"
            elif 0 == j and 0 == i:
                row = row + "O"
            elif area.get(j).get(i) is None:
                row = row + " "
            elif area.get(j).get(i) == 0:
                row = row + "#"
            elif area.get(j).get(i) == 1:
                row = row + "."
            else:
                row = row + "X"

        print(row)

def codeToDirection(code):
    if code == 1:
        return (0, 1)
    elif code == 2:
        return (0, -1)
    elif code == 3:
        return (-1, 0)
    else:
        return (1, 0)

def updateArea(area, position, code):
    if area.get(position[0]) is None:
        area.update( {position[0]: dict()} )
    area.get(position[0]).update( {position[1]: code} )
    return area

def calculateMove(robot, area):
    return randint(1, 4)

def calculateNextPath(robot, area):
    visited = dict( {robot[0]: dict( {robot[1]: 1} )} )
    paths = [(robot, deque([]))]
    newPaths = []

    while len(paths) > 0:
        for path in paths:
            for i in range(1, 5):
                displacement = codeToDirection(i)

                if area.get(path[0][0] + displacement[0]) is None or area.get(path[0][0] + displacement[0]).get(path[0][1] + displacement[1]) is None:
                    path[1].append(i)
                    return path[1]

                if area.get(path[0][0] + displacement[0]).get(path[0][1] + displacement[1]) == 0:
                    continue
                
                if visited.get(path[0][0] + displacement[0]) is None:
                    visited.update({path[0][0] + displacement[0]: dict()})
                if visited.get(path[0][0] + displacement[0]).get(path[0][1] + displacement[1]) is not None:
                    continue
                visited.get(path[0][0] + displacement[0]).update({path[0][1] + displacement[1]: 1})
                newPath = path[1].copy()
                newPath.append(i)
                newPaths.append(((path[0][0] + displacement[0], path[0][1] + displacement[1]), newPath))
        
        paths = newPaths
        newPaths = []
    
    return None

def AStar(start, end, area):
    visited = dict( {start[0]: dict( {start[1]: 1} )} )
    paths = [(start, 0)]

    while len(paths) > 0:
        paths.sort(key=lambda path: path[1] + (abs(end[0] - path[0][0])) + (abs(end[1] - path[0][1])))
        
        best = ((0,0), 1000000000)
        for i in range(1, 5):
            displacement = codeToDirection(i)
            newPos = (paths[0][0][0] + displacement[0], paths[0][0][1] + displacement[1])

            if area.get(newPos[0]).get(newPos[1]) == 2:
                displayMap(visited, end)
                return paths[0][1] + 1
            if area.get(newPos[0]).get(newPos[1]) == 0:
                continue
            if visited.get(newPos[0]) is None:
                visited.update({newPos[0]: dict()})
            if visited.get(newPos[0]).get(newPos[1]) is not None:
                continue
            
            h = (abs(end[0] - paths[0][0][0])) + (abs(end[1] - paths[0][0][1]))
            if h < best[1]:
                best = (newPos, h)
        
        if best[1] == 1000000000:
            paths = paths[1:]
        else:
            paths.append((best[0], paths[0][1] + 1))
            visited.get(best[0][0]).update({best[0][1]: 1})

    return None

cpu = IntcodeCPU([int(x) for i,x in enumerate(open("input.txt", "r").read().split(','))])

oxygenSystem = None
area = dict( {0: dict( {0: 1} )} )
robot = (0, 0)
lastDirection = (0, 0)

try:
    while True:
        path = calculateNextPath(robot, area)
        if path is None:
            break

        for move in path:
            lastDirection = codeToDirection(move)
            response = cpu.send(move)
            if response == 0:
                wall = (robot[0] + lastDirection[0], robot[1] + lastDirection[1])
                area = updateArea(area, wall, response)
            else:
                robot = (robot[0] + lastDirection[0], robot[1] + lastDirection[1])
                area = updateArea(area, robot, response)
                if response == 2:
                    oxygenSystem = robot
        
    displayMap(area, robot)
        
except ProgramEnded:
    print("An error occured")

print(AStar((0,0), oxygenSystem, area))