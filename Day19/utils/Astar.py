from __future__ import annotations

from heapq import heappush, heappop, heapify
from typing import Hashable, List, Tuple

class PotentialSolution:
    def cost(self) -> int: raise NotImplementedError( "Must implement the cost function" )
    def key(self) -> Hashable: raise NotImplementedError( "Must implement the key function" )
    def neighbors(self) -> List[PotentialSolution]: raise NotImplementedError( "Must implement the neighbors function" )
    def is_goal(self) -> bool: raise NotImplementedError( "Must implement the is_goal function" )

    def __eq__(self, other: PotentialSolution): return self.cost().__eq__(other.cost())
    def __ne__(self, other: PotentialSolution): return self.cost().__ne__(other.cost())
    def __lt__(self, other: PotentialSolution): return self.cost().__lt__(other.cost())
    def __le__(self, other: PotentialSolution): return self.cost().__le__(other.cost())
    def __gt__(self, other: PotentialSolution): return self.cost().__gt__(other.cost())
    def __ge__(self, other: PotentialSolution): return self.cost().__ge__(other.cost())
    def __hash__(self): return hash(self.key())


def solveAstar(initial_state: PotentialSolution) -> PotentialSolution:
    potentials: List[PotentialSolution] = [initial_state]
    visited = set([initial_state])
    heapify(potentials)

    while not potentials[0].is_goal():
        current = heappop(potentials)
        for neighbor in [n for n in current.neighbors() if n not in visited]:
            visited.add(neighbor)
            heappush(potentials, neighbor)
    
    return potentials[0]


# Tests
if __name__ == '__main__':
    fav = 1358
    class Path(PotentialSolution):
        moves: int
        goal: Tuple[int, int]
        position: Tuple[int, int]


        def  __init__(self,
                    goal: Tuple[int, int],
                    position: Tuple[int, int]):
            self.goal = goal
            self.position = position
            self.moves = 0

        def cost(self) -> int:
            return self.moves + abs(self.position[0] - self.goal[0]) + abs(self.position[1] - self.goal[1])

        def key(self) -> Hashable:
            return self.position

        def neighbors(self) -> List[Path]:
            nei = []
            for x, y in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                newPos = (self.position[0] + x, self.position[1] + y)
                intw = bin((newPos[0]*newPos[0] + 3*newPos[0] + 2*newPos[0]*newPos[1] + newPos[1] + newPos[1]*newPos[1]) + fav)[2:]
                if sum([int(i) for i in intw]) % 2 == 0:
                    new = Path(self.goal, newPos)
                    new.moves = self.moves + 1
                    nei.append(new)
            return nei

        def is_goal(self) -> bool:
            return self.position == self.goal
    
    s = Path((31, 39), (1, 1))
    assert solveAstar(s).moves == 96

    