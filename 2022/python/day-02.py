
def score(i_play, outcome):
    return i_play + 1 + outcome * 3

def part_1_score(they_play, i_play):
    outcome = (i_play - they_play + 1) % 3
    return score(i_play, outcome)

input = open('input-02', 'r').read().strip()
guide = [(ord(line[0]) - ord('A'), ord(line[2]) - ord('X')) for line in input.split('\n')]

print(f'Part1: {sum([part_1_score(*round) for round in guide])}')


def part_2_score(they_play, outcome):
    i_play = (they_play + outcome - 1) % 3
    return score(i_play, outcome)

print(f'Part2: {sum([part_2_score(*round) for round in guide])}')
