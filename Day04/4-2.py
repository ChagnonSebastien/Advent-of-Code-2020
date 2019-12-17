def valid(values):
    groups = [(values[0], 1)]

    for i in range(1, len(values)):
        if values[i-1] > values[i]:
            return False
        
        if groups[len(groups) - 1][0] == values[i]:
            groups[len(groups) - 1] = (values[i], groups[len(groups) - 1][1] + 1)
        else:
            groups.append((values[i], 1))
    
    for group in groups:
        if (group[1] == 2):
            return True
    
    return False


lowerBound = 136760
higherBound = 595730

results = []

for i in range(lowerBound, higherBound):
    values = [int(n) for n in str(i)]
    if valid(values):
        results.append(i)

print(len(results))
    
        