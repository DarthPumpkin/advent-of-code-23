import copy
import math
import random
from typing import Iterable, Sequence, TypeVar

import tqdm


INPUT_FILE = 'day25/input.txt'


Node = str
Edge = tuple[Node, Node]
T = TypeVar('T')


class Graph:
    def __init__(self):
        self.adjacency_lists: dict[Node, set[Node]] = {}
        self.edges: RandomMultiset[Edge] = RandomMultiset()

    def add_edge(self, n1: Node, n2: Node, multiplicity=1) -> None:
        n1, n2 = sorted((n1, n2))
        if (n1, n2) not in self.edges:
            l1 = self.adjacency_lists.get(n1, set())
            l1.add(n2)
            self.adjacency_lists[n1] = l1
            l2 = self.adjacency_lists.get(n2, set())
            l2.add(n1)
            self.adjacency_lists[n2] = l2
        self.edges.add((n1, n2), multiplicity)

    def remove_edge(self, n1: Node, n2: Node) -> int:
        n1, n2 = sorted((n1, n2))
        self.adjacency_lists[n1].remove(n2)
        if not self.adjacency_lists[n1]:
            del self.adjacency_lists[n1]
        self.adjacency_lists[n2].remove(n1)
        if not self.adjacency_lists[n2]:
            del self.adjacency_lists[n2]
        return self.edges.remove_all((n1, n2))

    def contract_edge(self, n1: Node, n2: Node) -> None:
        n1, n2 = sorted((n1, n2))
        new_node = f'{n1}-{n2}'
        self.remove_edge(n1, n2)
        neighbors1 = list(self.adjacency_lists.get(n1, set()))
        neighbors2 = list(self.adjacency_lists.get(n2, set()))
        for n in neighbors1:
            old_edge = tuple(sorted((n1, n)))
            multiplicity = self.remove_edge(*old_edge)
            new_edge = tuple(sorted((new_node, n)))
            self.add_edge(*new_edge, multiplicity) # type: ignore
        for n in neighbors2:
            old_edge = tuple(sorted((n2, n)))
            multiplicity = self.remove_edge(*old_edge)
            new_edge = tuple(sorted((new_node, n)))
            self.add_edge(*new_edge, multiplicity) # type: ignore


class RandomMultiset(Sequence[T]):
    """A set that allows for random access and removal of items."""
    def __init__(self, iterable: Iterable[T] = ()):
        self.items = list(iterable)
        self.item_indices = {item: i for i, item in enumerate(self.items)}
        self.item_counts = {item: 1 for item in self.items}

    def add(self, item: T, multiplicity=1) -> None:
        """Add an item to the set."""
        if not item in self.item_indices:
            self.items.append(item)
            self.item_indices[item] = len(self.items) - 1
            self.item_counts[item] = multiplicity
        else:
            self.item_counts[item] += multiplicity

    def remove_all(self, item: T) -> int:
        """Remove all instances of an item from the set.
        
        Args:
            item: The item to remove.
            
        Returns:
            The multiplicity of the item before removal."""
        if not item in self.item_indices:
            raise ValueError(f'Item {item} not in set.')
        multiplicity = self.item_counts[item]
        index = self.item_indices[item]
        swap_item = self.items[-1]
        self.items[index] = swap_item
        self.items.pop()
        self.item_indices[swap_item] = index
        del self.item_indices[item]
        del self.item_counts[item]
        return multiplicity

    def __getitem__(self, index: int) -> T:
        return self.items[index]

    def __len__(self) -> int:
        return len(self.items)

    def __iter__(self) -> Iterable[T]:
        return iter(self.items)

    def __contains__(self, item: T) -> bool:
        return item in self.item_indices

    def __repr__(self) -> str:
        return f'RandomSet({self.items})'


def karger_min_cut(graph: Graph, mincut_size: int, max_attempts=None) -> Edge:
    """Compute the min-cut of an undirected graph, provided that the min-cut size is known.
    
    Args:
        graph: The graph.
        mincut_size: The size of the min-cut.
        max_attempts: The maximum number of times to run the algorithm. If the min-cut is not found
        within the maximum number of attempts, a RuntimeError is raised. Defaults to
        comb(n, 2) * log(n) where n is the number of edges. This heuristic has a failure
        probability of at most 1/n.
    
    Returns:
        The edge that constitutes the min-cut. The nodes of the edge carry the names of its
        contracted nodes, separated by a hyphen (-)."""
    if max_attempts is None:
        _n = len(graph.edges)
        max_attempts = int(math.comb(_n, 2) * math.log(_n))
    for _ in tqdm.trange(max_attempts):
        current_graph = copy.deepcopy(graph)
        while len(current_graph.edges) > 1:
            edge = random.choice(current_graph.edges.items)
            current_graph.contract_edge(*edge)
        mincut = current_graph.edges[0]
        if current_graph.edges.item_counts[mincut] == mincut_size:
            return mincut
    raise RuntimeError(f'Min-cut not found within {max_attempts} attempts.')


def main():
    with open(INPUT_FILE, 'r') as fp:
        lines = [line.rstrip('\n') for line in fp]
    input_ = parse_input(lines)
    solution = solve(input_)
    print(solution)


def solve(graph: Graph) -> int:
    n1, n2 = karger_min_cut(graph, mincut_size=3)
    size1 = len(n1.split('-'))
    size2 = len(n2.split('-'))
    return size1 * size2


def parse_input(lines: Sequence[str]) -> Graph:
    graph = Graph()
    for line in lines:
        n1, others = line.split(': ')
        others = others.split(' ')
        for n2 in others:
            graph.add_edge(n1, n2)
    return graph


if __name__ == '__main__':
    main()
