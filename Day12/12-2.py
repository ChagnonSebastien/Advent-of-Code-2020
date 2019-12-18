class Planet1D:
    velocity = 0

    def __init__(self, position):
        self.position = position

    def simulate(self):
        self.position = self.position + self.velocity
    
    def updateVelocity(self, planets):
        change = 0
        for planet in planets:
            distance = planet.position - self.position
            change +=  0 if distance == 0 else int(distance / abs(distance))
        
        self.velocity += change
    
    def __str__(self):
        return str(self.position) + "   -=-   " + str(self.velocity)
    
    def __eq__(self, other):
        if self.position != other.position:
            return False
        if self.velocity != other.velocity:
            return False
        return True


def stripFormat(s):
    return s.replace(' ', '').replace('>', '').replace('<', '').replace('=', '').replace('x', '').replace('y', '').replace('z', '')

def ppcm(a, b):
    v = [a,b]
    v.sort()

    while True:
        v.sort()

        if v[1] % v[0] == 0:
            break
        
        v[1] = v[1] % v[0]
    
    return int(a * b / v[0])



data = [[Planet1D(int(p)) for p in stripFormat(planet.rstrip('\n')).split(',')] for planet in open("input.txt", "r")]
dimensions = [*zip(*data)]

compareSet = [[Planet1D(int(p)) for p in stripFormat(planet.rstrip('\n')).split(',')] for planet in open("input.txt", "r")]

firstRecurence = [0, 0, 0]

i = 0

while firstRecurence[0] == 0 or firstRecurence[1] == 0 or firstRecurence[2] == 0:
    for d in range(len(dimensions)):
        for planet in dimensions[d]:
            if firstRecurence[d] == 0:
                planet.updateVelocity(dimensions[d])
        for planet in dimensions[d]:
            if firstRecurence[d] == 0:
                planet.simulate()
        
    i += 1

    for d in range(len(dimensions)):
        found = True
        for p in range(len(dimensions[d])):
            if firstRecurence[d] != 0 or dimensions[d][p] != compareSet[p][d]:
                found = False
                break
        
        if found:
            firstRecurence[d] = i

print(ppcm(ppcm(firstRecurence[0], firstRecurence[1]), firstRecurence[2]))
