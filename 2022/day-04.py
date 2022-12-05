def is_contained(startA, endA, startB, endB):
    return int(startB) >= int(startA) and int(endB) <= int(endA) or int(startA) >= int(startB) and int(endA) <= int(endB)

input = open('input-04', 'r').read().strip()
assignements = input.split('\n')
pairs = [assignement.split(',') for assignement in assignements]
ranges = [(range1.split('-'), range2.split('-')) for range1, range2 in pairs]
contained = [is_contained(*range[0], *range[1]) for range in ranges]

print(f'Part1: {sum(contained)}')


def is_overlaping(startA, endA, startB, endB):
    return not (int(endA) < int(startB) or int(endB) < int(startA))

overlaping = [is_overlaping(*range[0], *range[1]) for range in ranges]

print(f'Part2: {sum(overlaping)}')