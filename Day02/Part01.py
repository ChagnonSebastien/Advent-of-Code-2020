from re import match, findall

def valid(instruction):
  (min, max, letter, password) = match(r'([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)', instruction).group(1, 2, 3, 4)
  count = len(findall(letter, password))
  return count >= int(min) and count <= int(max)

validPassword = [line for line in open("Day02/input", "r").read().strip().split("\n") if valid(line)]
print(len(validPassword))