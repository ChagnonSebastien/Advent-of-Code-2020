from collections import deque

def recursiveBattle(p1: deque, p2: deque, depth = 1):
    p1alreadySeen = set()
    p2alreadySeen = set()

    while len(p1) != 0 and len(p2) != 0:
        deck1hash = ",".join(str(c) for c in p1)
        deck2hash = ",".join(str(c) for c in p2)
        if deck1hash in p1alreadySeen or deck2hash in p2alreadySeen:
            return (1, 0)
        else:
            p1alreadySeen.add(deck1hash)
            p2alreadySeen.add(deck2hash)
        
        a = p1.popleft()
        b = p2.popleft()

        winner = 0
        if a <= len(p1) and b <= len(p2):
            winner, _ = recursiveBattle(deque(list(p1)[:a]), deque(list(p2)[:b]), depth + 1)
        else:
            winner = 1 if a > b else 2

        if winner == 1:
            p1.append(a)
            p1.append(b)
        else:
            p2.append(b)
            p2.append(a)

    winner = 1 if len(p2) == 0 else 2
    
    p1.reverse()
    p2.reverse()

    return (winner, sum([(i + 1) * x for i, x in enumerate(p1+p2)]))

data = open("Day22/input", "r").read().strip().split("\n\n")
p1 = deque([int(c) for c in data[0].split("\n")[1:]])
p2 = deque([int(c) for c in data[1].split("\n")[1:]])
_, score = recursiveBattle(p1, p2)
print(score)
