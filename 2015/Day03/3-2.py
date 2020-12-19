moves = [c for c in open("input.txt", "r").read()]

santaPos = (0,0)
robotPos = (0,0)
visited = [santaPos]

santaTurn = True

for c in moves:
    if c == "^":
        if santaTurn:
            santaPos = (santaPos[0], santaPos[1] + 1)
        else:
            robotPos = (robotPos[0], robotPos[1] + 1)
    if c == "v":
        if santaTurn:
            santaPos = (santaPos[0], santaPos[1] - 1)
        else:
            robotPos = (robotPos[0], robotPos[1] - 1)
    if c == ">":
        if santaTurn:
            santaPos = (santaPos[0] + 1, santaPos[1])
        else:
            robotPos = (robotPos[0] + 1, robotPos[1])
    if c == "<":
        if santaTurn:
            santaPos = (santaPos[0] - 1, santaPos[1])
        else:
            robotPos = (robotPos[0] - 1, robotPos[1])

    if santaTurn:
        if santaPos not in visited:
            visited.append(santaPos)
    else:
        if robotPos not in visited:
            visited.append(robotPos)
    
    santaTurn = not santaTurn

print(len(visited))
