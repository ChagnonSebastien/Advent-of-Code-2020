moves = [c for c in open("input.txt", "r").read()]

pos = (0,0)
visited = [pos]

for c in moves:
    if c == "^":
        pos = (pos[0], pos[1] + 1)
    if c == "v":
        pos = (pos[0], pos[1] - 1)
    if c == ">":
        pos = (pos[0] + 1, pos[1])
    if c == "<":
        pos = (pos[0] - 1, pos[1])

    if pos not in visited:
        visited.append(pos)

print(len(visited))
