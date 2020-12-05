from re import match

def valid(instruction):
  (pos1, pos2, letter, password) = match(r'([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)', instruction).group(1, 2, 3, 4)
  return (password[int(pos1)-1] == letter) != (password[int(pos2)-1] == letter)

validPassword = [line for line in open("Day02/input", "r").read().strip().split("\n") if valid(line)]
print(len(validPassword))
