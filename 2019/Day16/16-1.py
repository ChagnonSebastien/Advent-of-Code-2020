from math import floor

data = [int(i) for i in open("input.txt", "r").read().rstrip()]
pattern = [0, 1, 0, -1]

for n in range(100):
    newData = []
    for i in range(len(data)):
        v = 0
        for j in range(len(data)):
            v += data[j] * pattern[int(floor((j+1)/(i+1))) % 4]
        newData.append(int(str(v)[len(str(v))-1:]))
    data = newData

response = ""
for n in range(8):
    response = response + str(data[n])
print(response)