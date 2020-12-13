lines = [line for line in open("Day13/input", "r").read().strip().split("\n")]
early = int(lines[0])
buses = [int(n) for n in lines[1].split(',') if n != 'x']

while not any(early % b == 0 for b in buses):
    early += 1

print(next(b for b in buses if early % b == 0) * (early - int(lines[0])))
