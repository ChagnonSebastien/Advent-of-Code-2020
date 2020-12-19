numbers = open("Day15/input", "r").read().strip().split(',')
lastOccurence = { int(n): (i + 1) for i, n in enumerate(numbers[:len(numbers) - 1]) }
round = len(numbers) + 1
last = int(numbers[len(numbers) - 1])

while round <= 30000000:
    if last not in lastOccurence.keys():
        lastOccurence.update({ last: round - 1 })
        last = 0
    else:
        prev = lastOccurence.get(last)
        lastOccurence.update({ last: round - 1 })
        last = round - prev - 1

    round += 1

print(last)
