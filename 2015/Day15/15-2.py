import copy
import numpy as np

ingredients = np.array([[int(attribute.split()[1]) for attribute in line.rstrip().split(': ')[1].split(', ')] for line in open("input.txt", "r")])

class Recipe:

    def __init__(self, quantities=np.ones((ingredients.shape[0]), int)):
        self.quantities = quantities
    
    def get_stats(self):
        return np.dot(self.quantities, ingredients)
    
    def get_score(self):
        stats = self.get_stats()
        score = 1
        for stat in stats[:len(stats)-1]:
            score *= max(stat, 0)
        return score
    
    def get_calories(self):
        stats = self.get_stats()
        return stats[len(stats)-1]
    
    def __repr__(self):
        return str(self.quantities)


bestScore = 0
bestRecipe = None

for i in range(0, 101):
    for j in range(0, 101-i):
        for k in range(0, 101-(i+j)):
            l = 100 - (i + j + k)

            print("Testing Recipe: ", (i, j, k, l))
            potentialRecipe = Recipe([i,j,k,l])
            if potentialRecipe.get_calories() > 500:
                continue
            potentialScore = potentialRecipe.get_score()
            if potentialScore > bestScore:
                bestScore = potentialScore
                bestRecipe = potentialRecipe


print("\n\n=============Results===========")
print("Ingredients:")
print(ingredients)
print("\nRecipe:")
print(bestRecipe)
print("\nStats:")
print(bestRecipe.get_stats())
print("\nCalories: ", bestRecipe.get_calories())
print("\nScore: ", bestScore)