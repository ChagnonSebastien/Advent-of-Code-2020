from math import ceil

def parseChem(chem):
    parts = chem.split(' ')
    return(parts[1], int(parts[0]))

def missingSubstrate(quantity):
    for chem in quantity.keys():
        if chem != "ORE" and quantity.get(chem) < 0:
            return True
    
    return False

data = [orbit.rstrip('\n').split(' => ') for orbit in open("input.txt", "r")]


quantity = dict({"ORE": 0})
requirement = dict({"FUEL": []})
recipes = dict()
for reaction in data:
    inputChems = [parseChem(chem) for chem in reaction[0].split(', ')]
    outputChem = parseChem(reaction[1])

    if quantity.get(outputChem[0]) is None:
        quantity.update({outputChem[0]: 0})

    for chem in inputChems:
        if requirement.get(chem[0]) is None:
            requirement.update( {chem[0]: []} )
        
        requirement.get(chem[0]).append(outputChem[0])

    recipes.update({outputChem[0]: (outputChem[1], inputChems)})

delta = pow(2, 20)
fuel = delta

while True:
    print((delta, fuel))
    quantity.update( {"FUEL": -fuel, "ORE": 0} )

    while missingSubstrate(quantity):
        for chem in requirement.keys():
            if len(requirement.get(chem)) > 0 or quantity.get(chem) >= 0 or chem == "ORE":
                continue
            
            recipe = recipes.get(chem)
            multiplier = int(ceil(-quantity.get(chem) / recipe[0]))
            
            for ingredient in recipe[1]:
                quantity.update( {ingredient[0]: quantity.get(ingredient[0]) - (ingredient[1] * multiplier)} )
                requirement.update( {ingredient[0]: list(filter(lambda x: x != chem, requirement.get(ingredient[0])))} )

            quantity.update( {chem: quantity.get(chem) + (recipe[0] * multiplier)} )

    if -quantity.get("ORE") < 1000000000000:
        fuel += delta
    elif delta > 1:
        fuel -= delta
        delta = int(delta/2)
        fuel += delta
    else:
        fuel -= delta
        break

print(fuel)