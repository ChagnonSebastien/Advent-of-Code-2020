lines = [line.rstrip() for line in open("input.txt", "r").readlines()]

nice = []

for line in lines:
    twice = False
    sandwitch = False

    combinations = []

    lastChar = ''
    for c in line:
        
        combination = lastChar + c
        if combination in combinations[0:len(combinations)-1]:
            twice = True

        if len(combinations) >= 1 and combinations[len(combinations)-1][0] == c:
            sandwitch = True

        if len(combination) == 2:
            combinations.append(combination)

        lastChar = c
    
    if twice and sandwitch:
        nice.append(line)

print(len(nice))