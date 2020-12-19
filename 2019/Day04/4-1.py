def valid(values):
    sameAdjacent = False

    for j in range(len(values) - 1):
        if values[j] > values[j+1]:
            return False
        
        if values[j] == values[j+1]:
            sameAdjacent = True
    
    return sameAdjacent


lowerBound = 136760
higherBound = 595730

results = []

for i in range(lowerBound, higherBound):
    values = [int(n) for n in str(i)]
    if valid(values):
        results.append(i)

print(len(results))
    
        