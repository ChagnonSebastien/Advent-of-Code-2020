import copy

class Ingredient:

    def __init__(self, name, stats):
        self.name = name
        self.stats = stats
    
    def __repr__(self):
        return self.name + ": " + str(self.stats)


class Recipe:

    def __init__(self):
        self.content = dict()
    
    def add_ingredient(self, ingredient, ingredients):
        nextStep = copy.deepcopy(self)
        nextStep.content.update({ingredient.name: nextStep.content.get(ingredient.name, 0) + 1})
        return (nextStep, nextStep.calculate_score(ingredients))
    
    def calculate_stats(self, ingredients):
        stats = dict()
        for (ingredient, amount) in self.content.items():
            for (attribute, value) in ingredients.get(ingredient).stats.items():
                stats.update({attribute: stats.get(attribute, 0) + amount * value})
        return stats
    
    def calculate_score(self, ingredients):
        stats = self.calculate_stats(ingredients)
        score = 1

        for amount in stats.values():
            score *= amount
        
        return score
        
    
    def __repr__(self):
        return str(self.content)




data = [line.rstrip().split(': ') for line in open("input.txt", "r")]

ingredients = dict()
for line in data:
    attributes = [attribute.split() for attribute in line[1].split(', ')]
    stats = dict()
    for attribute in attributes:
        if attribute[0] != "calories":
            stats.update({attribute[0]: int(attribute[1])})

    ingredients.update({line[0]:Ingredient(line[0], stats)})

cookieRecipe = Recipe()
for ingredient in ingredients.values():
    cookieRecipe = cookieRecipe.add_ingredient(ingredient, ingredients)[0]


for _ in range(96):
    potentialRecipes = [cookieRecipe.add_ingredient(ingredient, ingredients) for ingredient in ingredients.values()]
    potentialRecipes.sort(key=lambda recipe: recipe[1], reverse=True)
    cookieRecipe = potentialRecipes[0][0]

print(cookieRecipe.calculate_score(ingredients))