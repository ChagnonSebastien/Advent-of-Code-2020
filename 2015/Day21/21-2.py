class Equipment:
    def __init__(self, cost, damage, armor):
        self.cost = cost
        self.damage = damage
        self.armor = armor

class Unit:
    def __init__(self, hp, damage, armor, equipementList: Equipment):
        self.cost = 0
        self.hp = hp
        self.damage = damage
        self.armor = armor
        for equipement in equipementList:
            self.cost += equipement.cost
            self.damage += equipement.damage
            self.armor += equipement.armor

    def takeHit(self, opDamage):
        d = opDamage - self.armor
        if d < 1:
            d = 1
        self.hp -= d


weapons = [
    Equipment(8, 4, 0),
    Equipment(10, 5, 0),
    Equipment(25, 6, 0),
    Equipment(40, 7, 0),
    Equipment(74, 8, 0),
]

armors = [
    Equipment(0, 0, 0),
    Equipment(13, 0, 1),
    Equipment(31, 0, 2),
    Equipment(53, 0, 3),
    Equipment(75, 0, 4),
    Equipment(102, 0, 5),
]

rings = [
    Equipment(25, 1, 0),
    Equipment(50, 2, 0),
    Equipment(100, 3, 0),
    Equipment(20, 0, 1),
    Equipment(40, 0, 2),
    Equipment(80, 0, 3),
]

max = 0
for weapon in weapons:
    for armor in armors:
        for i in range(0, len(rings)+1):
            for j in range(0, len(rings)+1):
                if i == j and i != 0:
                    continue

                equipmentList = [weapon, armor]
                if i != 0:
                    equipmentList.append(rings[i-1])
                if j != 0:
                    equipmentList.append(rings[j-1])

                Boss = Unit(100, 8, 2, [])
                Player = Unit(100, 0, 0, equipmentList)

                while Boss.hp > 0 and Player.hp > 0:
                    Boss.takeHit(Player.damage)
                    if (Boss.hp > 0):
                        Player.takeHit(Boss.damage)
                
                if Boss.hp > 0 and Player.cost > max:
                    max = Player.cost
print(max)


