def distances(position, parentData):
    distances = []
    
    i = parentData.get(position)
    distance = 0
    while True:
        distances.append((i, distance))

        if parentData.get(i) is None:
            break

        i = parentData.get(i)
        distance += 1
    
    return distances[::-1]

data = [orbit.rstrip('\n').split(')') for orbit in open("input.txt", "r")]

parentData = dict()
for orbit in data:
    parentData.update({orbit[1]: orbit[0]})

youDistances = distances("YOU", parentData)
sanDistances = distances("SAN", parentData)

i = 0
while youDistances[i+1][0] == sanDistances[i+1][0]:
    i += 1

print(youDistances[i][1] + sanDistances[i][1])