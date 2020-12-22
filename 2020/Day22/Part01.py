from collections import deque

data = open("Day22/input", "r").read().strip().split("\n\n")
p1 = deque([int(c) for c in data[0].split("\n")[1:]])
p2 = deque([int(c) for c in data[1].split("\n")[1:]])

while len(p1) != 0 and len(p2) != 0:
    a = p1.popleft()
    b = p2.popleft()
    if a > b:
        p1.append(a)
        p1.append(b)
    else:
        p2.append(b)
        p2.append(a)

p1.reverse()
p2.reverse()
print(sum([(i + 1) * x for i, x in enumerate(p1+p2)]))
