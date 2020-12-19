chars = [c for c in open("input.txt", "r").read()]

floor = 0
for char in chars:
    i += 1
    if char == "(":
        floor += 1
    elif char == ")":
        floor -= 1
print(floor)