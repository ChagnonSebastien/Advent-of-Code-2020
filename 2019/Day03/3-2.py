def move(position, direction):
    if direction[0] == 'U':
        return (position[0], position[1] + 1)
    elif direction[0] == 'R':
        return (position[0] + 1, position[1])
    elif direction[0] == 'L':
        return (position[0] - 1, position[1])
    else:
        return (position[0], position[1] - 1)

data = [[(direction[0], int(direction[1:])) for direction in wire.rstrip('\n').split(',')] for wire in open("input.txt", "r")]
path = dict()
position = (0,0)
distance = 0

for direction in data[0]:
    for i in range(direction[1]):
        position = move(position, direction[0])
        distance += 1
        if path.get(position[0]) is None:
            path.update({position[0]: dict()})

        if path.get(position[0]).get(position[1]) is None:
            path.get(position[0]).update({position[1]: distance})

intersections = []
position = (0,0)
distance = 0

for direction in data[1]:
    for i in range(direction[1]):
        position = move(position, direction[0])
        distance += 1
        if path.get(position[0]) is not None and path.get(position[0]).get(position[1]) is not None:
            intersections.append(distance + path.get(position[0]).get(position[1]))

intersections.sort()
print(intersections[0])