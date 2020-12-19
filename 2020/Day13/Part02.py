lines = [line for line in open("Day13/input", "r").read().strip().split("\n")]
buses = [(i, int(n)) for i, n in enumerate(lines[1].split(',')) if n != 'x']

t = 0
jump = 1
for i, n in buses:
    while (t + i) % n != 0:
        t += jump
    jump *= n

print(t)
