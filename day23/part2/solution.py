import itertools as it
from typing import Sequence

import numpy as np


INPUT_FILE = 'day23/input.txt'


Coordinate = tuple[int, int]
Graph = dict[Coordinate, set[Coordinate]]
WeightedEdge = tuple[Coordinate, int]
WeightedGraph = dict[Coordinate, set[WeightedEdge]]


def main():
    with open(INPUT_FILE, 'r') as fp:
        lines = [line.rstrip('\n') for line in fp]
    map = parse_input(lines)
    solution = solve(map)
    print(solution)


def solve(map: np.ndarray) -> int:
    neighbors, start_node, goal_node = build_graph(map)
    condensed_graph = condense_graph(neighbors)
    paths = all_paths(condensed_graph, start_node, goal_node)
    lengths = [sum(w for _, w in path) for path in paths]
    return max(lengths)


def build_graph(map: np.ndarray) -> tuple[dict[Coordinate, set[Coordinate]], Coordinate, Coordinate]:
    map_height, map_width = map.shape
    start_x = np.where(map[0, :] == '.')[0].item()
    goal_x = np.where(map[-1, :] == '.')[0].item()
    start_node = (0, start_x)
    goal_node = (map_height - 1, goal_x)
    graph = {}
    for y, x in it.product(range(map_height), range(map_width)):
        neighbors = set()
        if map[y, x] in '.<>^v':
            # Check up
            if y > 0 and map[y - 1, x] in '.<>^v':
                neighbors.add((y - 1, x))
            # Check right
            if x < map_width - 1 and map[y, x + 1] in '.<>^v':
                neighbors.add((y, x + 1))
            # Check down
            if y < map_height - 1 and map[y + 1, x] in '.<>^v':
                neighbors.add((y + 1, x))
            # Check left
            if x > 0 and map[y, x - 1] in '.<>^v':
                neighbors.add((y, x - 1))
        if neighbors:
            graph[(y, x)] = neighbors
    return graph, start_node, goal_node


def condense_graph(graph: Graph) -> WeightedGraph:
    condensed_graph: WeightedGraph = {n: {(m, 1) for m in graph[n]} for n in graph}
    for node in graph:
        neighbors = condensed_graph[node]
        if len(neighbors) == 2:
            (n1, w1), (n2, w2) = neighbors
            del condensed_graph[node]
            condensed_graph[n1].remove((node, w1))
            condensed_graph[n2].remove((node, w2))
            condensed_graph[n1].add((n2, w1 + w2))
            condensed_graph[n2].add((n1, w1 + w2))
    return condensed_graph


def all_paths(graph: WeightedGraph, start: Coordinate, goal: Coordinate) -> Sequence[Sequence[WeightedEdge]]:
    """Find all simple paths from start to goal in a undirected (possibly cyclic) weighted graph.
    
    Args:
        graph: A graph.
        start: The starting node.
        goal: The goal node.
    
    Returns:
        A list of paths from start to goal.
    """
    paths = []
    stack: list[list[WeightedEdge]] = [[(start, 0)]]
    while stack:
        path = stack.pop()
        path_end, path_len = path[-1]
        path_nodes = {n for n, _ in path}
        new_candidates = [path + [neighbor] for neighbor in graph[path_end] if neighbor[0] not in path_nodes]
        for new_path in new_candidates:
            if new_path[-1][0] == goal:
                paths.append(new_path)
            else:
                stack.append(new_path)
    return paths


def parse_input(lines: Sequence[str]) -> np.ndarray:
    array = np.array([list(line) for line in lines])
    return array


if __name__ == '__main__':
    main()
