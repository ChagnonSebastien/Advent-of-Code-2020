def isValid(p):
    return all(field in p for field in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])

lines = [line for line in open("Day04/input", "r").read().split("\n")]
bls = [i for i, line in enumerate(lines) if line == ""]
passports = [" ".join(lines[a+1:b]) for a, b in zip([-1] + bls, bls)]
valid = [True for p in passports if isValid(p)]
print(sum(valid))
