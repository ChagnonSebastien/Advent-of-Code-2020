from __future__ import annotations

from utils.Astar import PotentialSolution
from typing import Hashable, List, Tuple
from networkx import Graph
from networkx.algorithms import shortest_path

class GridPosition:
    x: int = 0
    y: int = 0

    def  __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other: GridPosition) -> bool:
        return self.x == other.x and self.y == other.y
    
    def __copy__(self) -> GridPosition:
        return GridPosition(self.x, self.y)
  
    def __str__(self) -> str:
        return "({x}, {y})".format(x=self.x, y=self.y)
   
    def __repr__(self) -> str:
        return (self.x, self.y)

    def get_neighbor(self, move: Tuple[int, int]) -> GridPosition:
        newPos = self.__copy__()
        newPos.x = self.x + move[0]
        newPos.y = self.y + move[1]
        return newPos

    def get_neighbors(self) -> List[GridPosition]:
        return [self.get_neighbor(move) for move in [(0, 1), (1, 0), (-1, 0), (0, -1)]]

class GridGraph:
    lines: List[str] = []
    wall_chars = str
    floor_chars = str

    def __init__(self, lines: List[str], wall_chars: str = "#", floor_chars: str = ". "):
        self.lines = lines
        self.wall_chars = wall_chars
        self.floor_chars = floor_chars
  
    def __str__(self) -> str:
        return "\n".join(self.lines)
   
    def __repr__(self) -> str:
        return self.lines

    def get_width(self) -> int:
        return 0 if len(self.lines) == 0 else len(self.lines[0])

    def get_height(self) -> int:
        return len(self.lines)
    
    def is_valid(self, pos: GridPosition) -> bool:
        return pos.x >= 0 and pos.x < self.get_width() and pos.y >= 0 and pos.y < self.get_height()

    def get_char(self, pos: GridPosition) -> bool:
        return None if not self.is_valid(pos) else self.lines[pos.y][pos.x]

    def is_walkable(self, pos: GridPosition) -> bool:
        return self.is_valid(pos) and self.get_char(pos) not in self.wall_chars

    def get_neighbors(self, position: GridPosition, allow_walls: bool = False) -> List[GridPosition]:
        return [pos for pos in position.get_neighbors() if self.is_valid(pos) and (allow_walls or self.is_walkable(pos))]
    
    def as_network(self, simplify=True, strip_dangling=False):
        net = Graph()
        for i in range(self.get_width()):
            for j in range(self.get_height()):
                pos = GridPosition(i, j)
                if self.is_walkable(pos):
                    net.add_node(pos.__repr__(), char=self.get_char(pos))
                    
        for node, _ in net.nodes.items():
            pos = GridPosition(node[0], node[1])
            for neighbor in self.get_neighbors(pos):
                net.add_edge(node, neighbor.__repr__(), length=1)
        

        if simplify:
            toDissolve = []
            for (node, degree) in net.degree:
                if degree != 2:
                    continue
                
                if net.nodes[node]['char'] in self.floor_chars:
                    toDissolve.append(node)

            for node in toDissolve:
                (a, b) = net.adj.get(node)
                net.add_edge(a, b, length=(net.adj.get(node).get(a).get('length') + net.adj.get(node).get(b).get('length')))
                net.remove_node(node)

        while strip_dangling:
            toRemove = []
            for (node, degree) in net.degree:
                if degree != 1:
                    continue
                
                if net.nodes[node]['char'] in self.floor_chars:
                    toRemove.append(node)

            for node in toRemove:
                net.remove_node(node)
            
            if len(toRemove) == 0:
                break

        return net
        
    def findChar(self, char):
        return next(GridPosition(next(x for x, c in enumerate(line) if c == char), y) for y, line in enumerate(self.lines) if char in line)

class GridPath(PotentialSolution):
    moves: int
    goal: GridPosition
    position: GridPosition
    gg: GridGraph


    def  __init__(self,
                  goal: GridPosition,
                  position: GridPosition,
                  gg: GridGraph):
        self.goal = goal
        self.position = position
        self.moves = 0
        self.gg = gg

    def cost(self) -> int:
        return self.moves + abs(self.position.x - self.goal.x) + abs(self.position.y - self.goal.y)

    def key(self) -> Hashable:
        return self.position.__repr__()

    def neighbors(self) -> List[GridPath]:
        nei = []
        for newPos in gg.get_neighbors(self.position):
            gp = GridPath(self.goal, newPos, self.gg)
            gp.moves = self.moves + 1
            nei.append(gp)
        return nei

    def is_goal(self) -> bool:
        return self.position == self.goal



# Tests
if __name__ == '__main__':
    lines = [
        ".#.#.",
        "O#...",
        "...#.",
        "####.",
        "X....",
    ]
    gg = GridGraph(lines)
    

    assert solveAstar(GridPath(gg.findChar("X"), gg.findChar("O"), gg)).moves == 13


    net = gg.as_network(simplify=True, strip_dangling=True)
    path = shortest_path(
        net,
        next(x for x,y in net.nodes(data=True) if y['char']=="O"),
        next(x for x,y in net.nodes(data=True) if y['char']=="X"),
        'length'
    )
    assert sum([net.edges[a, b]['length'] for a, b in zip(path, path[1:])]) == 13
