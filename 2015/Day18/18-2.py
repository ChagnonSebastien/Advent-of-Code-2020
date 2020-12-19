import numpy as np

def getNeighbors(coordinates, grid):
    neighbors = []
    for displacement in [(1,1), (1,0), (1,-1), (0,1), (0,-1), (-1,1), (-1,0), (-1,-1)]:
        newCoordinates = (coordinates[0] + displacement[0], coordinates[1] + displacement[1])
        if newCoordinates[0] < 0 or newCoordinates[1] < 0 or newCoordinates[0] >= grid.shape[0] or newCoordinates[1] >= grid.shape[1]:
            continue

        neighbors.append(newCoordinates)
    
    return neighbors

grid = np.array([[True if c == '#' else False for c in line.rstrip()] for line in open("input.txt", "r")])
grid[0][0] = True
grid[0][grid.shape[1]-1] = True
grid[grid.shape[0]-1][0] = True
grid[grid.shape[0]-1][grid.shape[1]-1] = True

for _ in range(100):
    newGrid = np.zeros(grid.shape, bool)
    for i in range(len(newGrid)):
        for j in range(len(newGrid[i])):
            litNeighbors = 0
            for neighbor in getNeighbors((i, j), grid):
                if grid[neighbor[0]][neighbor[1]]:
                    litNeighbors += 1
            newGrid[i][j] = litNeighbors == 3 or (grid[i][j] and litNeighbors == 2)
    
    newGrid[0][0] = True
    newGrid[0][newGrid.shape[1]-1] = True
    newGrid[newGrid.shape[0]-1][0] = True
    newGrid[newGrid.shape[0]-1][newGrid.shape[1]-1] = True
    grid = newGrid

print(sum(sum(grid)))