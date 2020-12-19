width = 25
height = 6

def stats(layer):
    ones = 0
    twos = 0
    zeros = 0

    for char in layer:
        if char == '1':
            ones += 1
        elif char == '2':
            twos += 1
        else:
            zeros += 1

    return (ones, twos, zeros)

data = open("input.txt", "r").read().rstrip()
layers = [data[i*width*height:(i+1)*width*height] for i in range(int(len(data)/(width * height)))]

layerStats = list(map(stats, layers))
layerStats.sort(key=lambda stats:stats[2])
print(layerStats[0][0]*layerStats[0][1])