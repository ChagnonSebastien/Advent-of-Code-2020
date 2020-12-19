lines = [line for line in open("Day06/input", "r").read().split("\n")]
bls = [i for i, line in enumerate(lines) if line == ""]
groups = [lines[a+1:b] for a, b in zip([-1] + bls, bls)]
allAnswered = [set(g[0]).intersection(*g) for g in groups]
print(sum(len(a) for a in allAnswered))