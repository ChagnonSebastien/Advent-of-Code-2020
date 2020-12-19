lines = [line.rstrip() for line in open("input.txt", "r").readlines()]

nice = []

for line in lines:
    vowelCount = 0
    twice = False
    disallowedSubstring = False

    lastChar = ''
    for c in line:
        if c in "aeiou":
            vowelCount += 1

        if c == lastChar:
            twice = True
        
        combination = lastChar + c
        if combination == "ab" or combination == "cd" or combination == "pq" or combination == "xy":
            disallowedSubstring = True
        
        lastChar = c
    
    if vowelCount >= 3 and twice and not disallowedSubstring:
        nice.append(line)

print(len(nice))