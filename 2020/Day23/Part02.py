cups = [int(cup) for cup in open("Day23/input", "r").read().strip()]
for i in range(max(cups), 1_000_000):
    cups.append(i + 1)

nextCup = { a: b for a, b in zip(cups, cups[1:] + [cups[0]]) }
current = cups[0]

i = 0
while i < 10_000_000:
    i += 1

    firstToRemove = nextCup.get(current)
    secondToRemove = nextCup.get(firstToRemove)
    thirdToRemove = nextCup.get(secondToRemove)
    firstAfter = nextCup.get(thirdToRemove)

    destination = current - 1 if current > 1 else len(cups)
    while destination == firstToRemove or destination == secondToRemove or destination == thirdToRemove:
        destination = destination - 1 if destination > 1 else len(cups)
    
    afterDestination = nextCup.get(destination)

    nextCup.update({
        current: firstAfter,
        destination: firstToRemove,
        thirdToRemove: afterDestination,
    })

    current = firstAfter

a = nextCup.get(1)
b = nextCup.get(a)
print(a*b)
