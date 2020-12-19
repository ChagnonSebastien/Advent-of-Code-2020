data = [int(x) for x in open("input.txt", "r").read().split(',')]
data[1] = 12
data[2] = 2

i = 0
while data[i] != 99:
    if data[i] == 1:
        data[data[i+3]] = data[data[i+1]] + data[data[i+2]]
    else:
        data[data[i+3]] = data[data[i+1]] * data[data[i+2]]

    i += 4

print(data[0])