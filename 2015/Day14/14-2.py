
class Reindeer:
    def __init__(self, name, speed, flyTime, restTime):
        self.name = name
        self.speed = speed
        self.flyTime = flyTime
        self.restTime = restTime
        
        self.points = 0
        self.distance = 0
        self.isFlying = True
        self.changeIn = flyTime
    
    def tick(self):
        if self.isFlying:
            self.distance += self.speed
        
        self.changeIn -= 1
        
        if self.changeIn == 0:
            if self.isFlying:
                self.isFlying = False
                self.changeIn = self.restTime
            else:
                self.isFlying = True
                self.changeIn = self.flyTime
    
    def award_point(self):
        self.points += 1
    
    def __str__(self):
        return str({
            "name": self.name,
            "speed": self.speed,
            "flyTime": self.flyTime,
            "restTime":self.restTime,
            "distance": self.distance,
            "isFlying": self.isFlying,
            "changeIn": self.changeIn,
            "points": self.points
        })


infos = [line.split() for line in open("input.txt", "r")]

reindeers = []
for line in infos:
    reindeers.append(Reindeer(line[0], int(line[3]), int(line[6]), int(line[13])))

for _ in range(2503):
    for reindeer in reindeers:
        reindeer.tick()
    
    reindeers.sort(key=lambda reindeer: reindeer.distance, reverse=True)
    awardDistance = reindeers[0].distance
    for reindeer in reindeers:
        if reindeer.distance == awardDistance:
            reindeer.award_point()

reindeers.sort(key=lambda reindeer: reindeer.points, reverse=True)
print(reindeers[0].points)