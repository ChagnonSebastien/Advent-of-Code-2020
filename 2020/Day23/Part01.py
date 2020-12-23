cups = [int(cup) for cup in open("Day23/input", "r").read().strip()]
i = 0
while i < 100:
    i += 1
    
    current = cups[0]
    removed = cups[1:4]
    remaining = cups[4:]

    destination = current - 1 if current > 1 else len(cups)
    while destination not in remaining:
        destination = destination - 1 if destination > 1 else len(cups)
    
    destinationIndex = next(i for i, c in enumerate(remaining) if c == destination)
    cups = remaining[:destinationIndex+1] + removed + remaining[destinationIndex+1:] + [current]

oneIndex = next(i for i, c in enumerate(cups) if c == 1)
cups = [str(c) for c in cups]
print("".join(cups[oneIndex+1:] + cups[:oneIndex]))
