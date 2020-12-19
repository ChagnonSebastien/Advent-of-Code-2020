import numpy as np
from math import ceil

nCards = 10007

def splitInstructions(line):
    if line == "deal into new stack":
        return (0, 0)
    else:
        words = line.split(" ")
        return (1 if words[0] == "cut" else 2, int(words[len(words)-1]) % nCards)

instructions = [module.rstrip('\n') for module in open("input.txt", "r")]
instructions = list(map(splitInstructions, instructions))

deck = np.array([i for i in range(nCards)])

for (technique, value) in instructions:

    if technique == 0:
        deck = np.flip(deck)
    elif technique == 1:
        deck = np.concatenate((deck[value:], deck[:value]))
    else:
        newDeck = np.empty(len(deck), dtype=int)
        i = 0
        while i < len(deck):
            newDeck[(i * value) % len(deck)] = deck[i]
            i += 1
        deck = newDeck


for i in range(len(deck)):
    if deck[i] == 2019:
        print(i)