data = open("Day25/input", "r").read().strip()
card = int(data.split('\n')[0])
door = int(data.split('\n')[1])

itself = 1
i = 0
while itself != card:
    i += 1
    itself = (itself * 7) % 20201227

itself = 1
for _ in range(i):
    itself = (itself * door) % 20201227

print(itself)
