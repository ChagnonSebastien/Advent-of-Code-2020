data = [orbit.rstrip('\n').split(')') for orbit in open("input.txt", "r")]

parentData = dict()
for orbit in data:
    parentData.update({orbit[1]: orbit[0]})

totalAmount = 0
for orbit in data:

    totalAmount += 1
    root = orbit[0]

    while parentData.get(root) is not None:
        root = parentData.get(root)
        totalAmount += 1

print(totalAmount)