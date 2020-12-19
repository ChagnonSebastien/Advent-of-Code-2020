from math import floor

modules = [int(module.rstrip('\n')) for module in open("input.txt", "r")]

fuelRequirement = 0
for module in modules:
    fuelRequirement += floor(module/3) - 2

print(fuelRequirement)