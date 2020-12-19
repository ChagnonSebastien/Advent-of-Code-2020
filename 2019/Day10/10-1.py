def shadow(station, asteroid, limitCol, limitRow):
    hidden = [station]

    distance = (asteroid[0] - station[0], asteroid[1] - station[1])
    reduced = True
    while reduced:
        reduced = False
        for i in range(2, max(abs(distance[0]), abs(distance[1]))+1):
            if distance[0] % i == 0 and distance[1] % i == 0:
                distance = (int(distance[0]/i), int(distance[1]/i))
                reduced = True

    #print(distance)
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

data = [[True if col == '#' else False for col in row.rstrip('\n')] for row in open("input.txt", "r")]
asteroids = getAllPositions(data)

bestView = 0
for station in asteroids:
    map = [[True if col == '#' else False for col in row.rstrip('\n')] for row in open("input.txt", "r")]
    visibility = visibilityMap(station, asteroids, map)
    q = len(getAllPositions(visibility))
    if q > bestView:
        bestView = q

print(bestView)
