import numpy as np
import re

memory = [line.strip() for line in open("input.txt", "r")]

encoded = [re.sub(r'\\', r'\\\\', line) for line in memory]
encoded = [re.sub(r'\"', r'\"', line) for line in encoded]
encoded = ['\"' + line + '\"' for line in encoded]

lenMemory = np.sum([len(s) for s in memory])
lenEncoded = np.sum([len(s) for s in encoded])

print(lenEncoded - lenMemory)