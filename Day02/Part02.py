from re import findall

def valid(instruction):
  (pos1, pos2, letter, password) = findall(r'([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)', instruction)[0]
  return (password[int(pos1)-1] == letter) != (password[int(pos2)-1] == letter)

validPassword = [line for line in open("Day02/input", "r").read().strip().split("\n") if valid(line)]
print(len(validPassword))
