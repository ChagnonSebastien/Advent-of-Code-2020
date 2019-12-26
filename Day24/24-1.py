class Vector:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def __str__(self):
        return( "({:d}, {:d})".format(self.x, self.y))
    
    def __repr__(self):
        return( "({:d}, {:d})".format(self.x, self.y))
    
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def adjacent(self):
        return [Vector(self.x + d.x, self.y + d.y) for d in [Vector(0, 1), Vector(0, -1), Vector(1, 0), Vector(-1, 0)]]


def isOnMap(coordinate, bugs):
    if coordinate.x < 0 or coordinate.y < 0:
        return False
    if coordinate.x >= len(bugs) or coordinate.y >= len(bugs[coordinate.x]):
        return False
    return True


def biodiversityRating(bugs):
    rating = 0
    for i in range(len(bugs)):
        for j in range(len(bugs[i])):
            if bugs[i][j] == "#":
                rating += pow(2, len(bugs[i]) * i + j)
    return rating


bugs = [module.rstrip('\n') for module in open("input.txt", "r")]
pastRatings = []

while biodiversityRating(bugs) not in pastRatings:
    pastRatings.append(biodiversityRating(bugs))

    next = []
    for x in range(len(bugs)):
        line = ""
        for y in range(len(bugs[x])):
            adjacentAlive = 0
            for tile in Vector(x, y).adjacent():
                if not isOnMap(tile, bugs):
                    continue
                
                if bugs[tile.x][tile.y] == "#":
                    adjacentAlive += 1
            
            preinfected = bugs[x][y] == "#"
            if adjacentAlive == 1:
                line = line + "#"
            elif adjacentAlive == 2 and not preinfected:
                line = line + "#"
            else:
                line = line + "."
        next.append(line)
    bugs = next

print(biodiversityRating(bugs))