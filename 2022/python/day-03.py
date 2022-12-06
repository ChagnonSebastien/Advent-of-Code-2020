def find_error(sack):
    middle = int(len(sack)/2);
    left = set(sack[:middle]);
    right = set(sack[middle:])
    return left.intersection(right).pop()

def get_priority(letter):
    if ord(letter) >= ord('a') and ord(letter) <= ord('z'):
        return ord(letter) - ord('a') + 1
    else:
        return ord(letter) - ord('A') + 27

input = open('input-03', 'r').read().strip()
sacks = input.split('\n');
errors = [find_error(sack) for sack in sacks]

print(f'Part1: {sum([get_priority(error) for error in errors])}')


def get_badge(sack1, sack2, sack3):
    return set(sack1).intersection(set(sack2)).intersection(set(sack3)).pop()

groups = [sacks[n:n+3] for n in range(0, len(sacks), 3)]
badges = [get_badge(*group) for group in groups]

print(f'Part2: {sum([get_priority(badge) for badge in badges])}')
