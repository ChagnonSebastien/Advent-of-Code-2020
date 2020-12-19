class Planet:
    velocity = [0, 0, 0]

    def __init__(self, position):
        self.position = position

    def simulate(self):
        self.position = [p + self.velocity[i] for i,p in enumerate(self.position)]
    
    def updateVelocity(self, planets):
        change = [0, 0, 0]
        for planet in planets:
            distance = [p - self.position[i] for i,p in enumerate(planet.position)]
            normalized = list(map(lambda d: 0 if d == 0 else int(d / abs(d)), distance))
            change = [c + normalized[i] for i,c in enumerate(change)]
        
        self.velocity = [self.velocity[i] + c for i,c in enumerate(change)]
    
    def potentialEnergy(self):
        e = 0
        for p in self.position:
            e += abs(p)
        return e
    
    def kineticEnergy(self):
        e = 0
        for v in self.velocity:
            e += abs(v)
        return e
    
    def totalEnergy(self):
        return self.potentialEnergy() * self.kineticEnergy()
    
    def __str__(self):
        return str(self.position) + "   -=-   " + str(self.velocity)


def stripFormat(s):
    return s.replace(' ', '').replace('>', '').replace('<', '').replace('=', '').replace('x', '').replace('y', '').replace('z', '')


planets = [Planet([int(p) for p in stripFormat(planet.rstrip('\n')).split(',')]) for planet in open("input.txt", "r")]

for i in range(1000):
    for planet in planets:
        planet.updateVelocity(planets)
    for planet in planets:
        planet.simulate()

systemEnergy = 0
for planet in planets:
    systemEnergy += planet.totalEnergy()

print(systemEnergy)