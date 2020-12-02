houses = {}

i = 0
while houses.get(i, 0) + i * 11 < 36000000:
    for j in range(1, 51):
        houses.update({i * j : houses.get(i * j, 0) + 11 * i})
    i+=1
print(i)