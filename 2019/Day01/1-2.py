from math import floor

modules = [int(module.rstrip('\n')) for module in open("input.txt", "r")]

fuelRequirement = 0
for module in modules:
    gazNeeded = module
    while True:
        gazNeeded = floor(gazNeeded/3) - 2

        if gazNeeded <= 0:
            break

        fuelRequirement += gazNeeded

print(fuelRequirement)