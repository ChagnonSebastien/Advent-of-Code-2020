lines = [line for line in open("Day06/input", "r").read().split("\n")]
bls = [i for i, line in enumerate(lines) if line == ""]
groups = [set("".join(lines[a+1:b])) for a, b in zip([-1] + bls, bls)]
print(sum(len(a) for a in groups))