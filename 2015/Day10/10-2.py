import re

def say(sequence):
    return str(len(sequence)) + sequence[0]


lookAndSay = "1113222113"

for _ in range(50):
    lookAndSay = "".join([say(l) for l in re.findall('(1+|2+|3+|4+|5+|6+|7+|8+|9+|0+)', lookAndSay)])

print(len(lookAndSay))