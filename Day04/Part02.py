from re import search

def isValid(p):
    return (
        search(r'byr:19[2-9][0-9]|200[0-2]', p) and 
        search(r'iyr:201[0-9]|2020', p) and
        search(r'eyr:202[0-9]|2030', p) and
        search(r'hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)', p) and
        search(r'hcl:#[0-9a-f]{6}(?![0-9a-f])', p) and
        search(r'(ecl:(amb|blu|brn|gry|grn|hzl|oth))', p) and
        search(r'pid:[0-9]{9}(?![0-9])', p)
    )

lines = [line for line in open("Day04/input", "r").read().split("\n")]
bls = [i for i, line in enumerate(lines) if line == ""]
passports = [" ".join(lines[a+1:b]) for a, b in zip([-1] + bls, bls)]
valid = [True for p in passports if isValid(p)]
print(sum(valid))