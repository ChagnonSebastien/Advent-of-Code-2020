input = open('input-01', 'r').read().strip()
elves = input.split('\n\n')
pockets = [[int(item) for item in items.split('\n')] for items in elves]
calories = [sum(items) for items in pockets]

print(f'Part1: {max(calories)}')
print(f'Part2: {sum(sorted(calories)[-3:])}')
