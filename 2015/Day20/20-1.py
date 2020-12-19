i = 0
presents = 10
while presents < 36000000:
    i += 2*3*4*5*6
    presents = 0
    for n in range(1, i+1):
        if i % n == 0:
            presents += 10 * n
    print(i, presents)