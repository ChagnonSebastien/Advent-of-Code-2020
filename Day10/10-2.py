from math import sqrt
from math import atan2
from math import pi
from collections import deque

def reduceVector(vector):
    distance = vector
    reduced = True
    while reduced:
        reduced = False
        for i in range(2, max(abs(distance[0]), abs(distance[1]))+1):
            if distance[0] % i == 0 and distance[1] % i == 0:
                distance = (int(distance[0]/i), int(distance[1]/i))
                reduced = True
    
    return distance

def shadow(station, asteroid, limitCol, limitRow):
    hidden = [station]

    distance = (asteroid[0] - station[0], asteroid[1] - station[1])
    distance = reduceVector(distance)

    position = asteroid
    while True:
        position = (position[0] + distance[0], position[1] + distance[1])
        if (not position[0] >= 0) or (not position[0] < limitRow) or (not position[1] >= 0) or (not position[1] < limitCol):
            break
        
        hidden.append(position)
    
    return hidden

def visibilityMap(station, asteroids, map):
    for asteroid in asteroids:
        if asteroid[0] == station[0] and asteroid[1] == station[1]:
            continue
        
        for blindSpot in shadow(station, asteroid, len(map[0]), len(map)):
            map[blindSpot[0]][blindSpot[1]] = False
    
    return map

def getAllPositions(map):
    asteroids = []
    for i in range(len(map)):
        for j in range(len(map[i])):
            if map[i][j]:
                asteroids.append((i, j))

    return asteroids

def angle(vector):
    result = (atan2(vector[0], vector[1]) - atan2(-1, 0)) * 180 / pi
    return result if result >=0 else result + 360

data = [[True if col == '#' else False for col in row.rstrip('\n')] for row in open("input.txt", "r")]
asteroids = getAllPositions(data)

bestView = 0
bestStation = None
for station in asteroids:
    map = [[True if col == '#' else False for col in row.rstrip('\n')] for row in open("input.txt", "r")]
    visibility = visibilityMap(station, asteroids, map)
    q = len(getAllPositions(visibility))
    if q > bestView:
        bestView = q
        bestStation = station

laserData = []
for asteroid in asteroids:
    distanceRow = asteroid[0] - bestStation[0]
    distanceCol = asteroid[1] - bestStation[1]
    if distanceCol == 0 and distanceRow == 0:
        continue

    distanceAbs = sqrt(pow(distanceRow, 2) + pow(distanceCol, 2))
    reducedVector = reduceVector((distanceRow, distanceCol))
    laserData.append((asteroid, reducedVector, distanceAbs))

rays = dict()
for data in laserData:
    if rays.get(data[1]) is None:
        rays.update({data[1]:[]})
    
    rays.get(data[1]).append((data[0], data[2]))

sequence = []
for key in rays.keys():
    ray = rays.get(key)
    ray.sort(key=lambda data: data[1])

    order = []
    for asteroid in ray:
        order.append(asteroid[0])
    
    sequence.append((angle(key), deque(order)))

sequence.sort(key=lambda ray: ray[0])

n = 0
while n < 200:
    for ray in sequence:
        if len(ray[1]) > 0:
            vaporized = ray[1].popleft()
            n += 1

            if n == 200:
                print(vaporized[1] * 100 + vaporized[0])

