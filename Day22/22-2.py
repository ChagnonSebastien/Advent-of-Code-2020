import numpy as np
from math import ceil

nCards = 119315717514047

def splitInstructions(line):
    if line == "deal into new stack":
        return (0, 0)
    else:
        words = line.split(" ")
        return (1 if words[0] == "cut" else 2, int(words[len(words)-1]) % nCards)

instructions = [module.rstrip('\n') for module in open("input.txt", "r")]
instructions = list(map(splitInstructions, instructions))

card = 2019
k = 0
while card != 2019 or k == 0:
    k+=1

    for n in range(1, len(instructions)+1):
        (technique, value) = instructions[len(instructions) - n]

        if technique == 0:
            card = nCards - card - 1
        elif technique == 1:
            card = (value + card) % nCards
        else:
            layer = 0
            for i in range(value):
                if (i * nCards + card) % value == 0:
                    layer = i
                    break
            card = int((layer * nCards + card) / value)

print(k)
