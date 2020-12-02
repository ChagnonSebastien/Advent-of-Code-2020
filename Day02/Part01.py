from re import findall

def valid(instruction):
  (min, max, letter, password) = findall(r'([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)', instruction)[0]
  count = len(findall(letter, password))
  return count >= int(min) and count <= int(max)

validPassword = [line for line in open("Day02/input", "r").read().strip().split("\n") if valid(line)]
print(len(validPassword))