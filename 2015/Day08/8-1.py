import numpy as np
import re

code = [line.strip() for line in open("input.txt", "r")]

memory = [re.sub(r'(\\\\)|(\\[x][0-9a-f]{2})|(\\\")', "!", line[1:len(line)-1]) for line in code]

lenCode = np.sum([len(s) for s in code])
lenMemory = np.sum([len(s) for s in memory])


print(lenCode - lenMemory)