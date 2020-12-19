from typing import List


class Spell:
    def __init__(self, cost, damage, heal, resistance, poison, manaPlus):
        self.cost = cost
        self.damage = damage
        self.heal = heal
        self.resistance = resistance
        self.poison = poison
        self.manaPlus = manaPlus

spells: List[Spell] = [
    Spell(53, 4, 0, False, False, False),
    Spell(73, 2, 2, False, False, False),
    Spell(113, 0, 0, True, False, False),
    Spell(173, 0, 0, False, True, False),
    Spell(229, 0, 0, False, False, True),
]

class Unit:
    def __init__(self, hp, mana, resistanceTurnsLeft, ManaPlusTurnsLeft, PoisonTurnsLeft, manaUsed, armor):
        self.hp = hp
        self.mana = mana
        self.resistanceTurnsLeft = resistanceTurnsLeft
        self.ManaPlusTurnsLeft = ManaPlusTurnsLeft
        self.PoisonTurnsLeft = PoisonTurnsLeft
        self.manaUsed = manaUsed
        self.armor = armor


    def takeSpell(self, spell: Spell):
        self.hp -= spell.damage if spell.damage > 0 else 0
        if spell.poison:
            self.PoisonTurnsLeft = 6


    def dealSpell(self, spell: Spell):
        self.manaUsed += spell.cost
        self.hp += spell.heal
        self.mana -= spell.cost
        if spell.manaPlus:
            self.ManaPlusTurnsLeft = 5
        if spell.resistance:
            self.resistanceTurnsLeft = 6

    def startTurn(self):
        if self.PoisonTurnsLeft >= 1:
            self.hp -= 3
        if self.ManaPlusTurnsLeft >= 1:
            self.mana += 101
        self.resistanceTurnsLeft -= 1
        self.ManaPlusTurnsLeft -= 1
        self.PoisonTurnsLeft -= 1
    
    def endTurn(self):
        self.armor = 7 if self.resistanceTurnsLeft >= 1 else 0
    
    def takeHit(self):
        d = 9 - self.armor
        if d < 1:
            d = 1
        self.hp -= d
    
    def availableSpells(self):
        return [s for s in spells if self.mana > s.cost]
    
    def copyDeep(self):
        return Unit(self.hp, self.mana, self.resistanceTurnsLeft, self.ManaPlusTurnsLeft, self.PoisonTurnsLeft, self.manaUsed, self.armor)


class BattleGround:
    def __init__(self, Player, Boss, turns):
        self.Player: Unit = Player
        self.Boss: Unit = Boss
        self.turns = turns
    
    def children(self):
        self.turns += 1
        self.Player.hp -= 1
        if self.Player.hp <= 0:
            return [self]

        self.Player.startTurn()
        self.Boss.startTurn()
        if (self.Boss.hp <= 0):
            return [self]

        c: List[BattleGround] = []
        for spell in self.Player.availableSpells():
            newBattleGround = BattleGround(self.Player.copyDeep(), self.Boss.copyDeep(), self.turns)
            newBattleGround.Player.dealSpell(spell)
            newBattleGround.Boss.takeSpell(spell)
            newBattleGround.Player.endTurn()
            newBattleGround.Boss.endTurn()
            if newBattleGround.Boss.hp > 0:
                
                newBattleGround.Player.hp -= 1
                if newBattleGround.Player.hp >= 0:
                    newBattleGround.turns += 1
                    newBattleGround.Boss.startTurn()
                    newBattleGround.Player.startTurn()
                    if newBattleGround.Boss.hp > 0:
                        newBattleGround.Player.takeHit()
                        newBattleGround.Boss.endTurn()
                        newBattleGround.Player.endTurn()
            
            c.append(newBattleGround)
        return c
        

            

minBg = BattleGround(Unit(0, 0, 0, 0, 0, 100000000000000, 0), Unit(0, 0, 0, 0, 0, 0, 0), 0)
battleGrounds= [BattleGround(Unit(50, 500, 0, 0, 0, 0, 0), Unit(58, 0, 0, 0, 0, 0, 0), 0)]
while len(battleGrounds) > 0:
    print(len(battleGrounds))
    newBG: List[BattleGround] = []
    for bg in battleGrounds:
        newBG.extend(bg.children())
    playerWon = [b for b in newBG if b.Boss.hp <= 0]
    for game in playerWon:
        if game.Player.manaUsed < minBg.Player.manaUsed:
            minBg = game

    battleGrounds = [b for b in newBG if b.Player.hp > 0 and b.Boss.hp > 0 and b.Player.manaUsed + 53 < minBg.Player.manaUsed]

print("Boss", minBg.Boss.hp)
print("Player", minBg.Player.hp)
print("Min mana:", minBg.Player.manaUsed)
print("Turns:", minBg.turns)
