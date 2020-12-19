lines = [line for line in open("Day05/input", "r").read().strip().split("\n")]
ids = [int(line.replace('F', "0").replace('B', "1").replace('L', "0").replace('R', "1"), 2) for line in lines]
print(max(ids))