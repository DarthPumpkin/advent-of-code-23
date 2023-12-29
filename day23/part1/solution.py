import itertools as it
from typing import Sequence

import numpy as np


INPUT_FILE = 'day23/input.txt'


Coordinate = tuple[int, int]
Graph = dict[Coordinate, set[Coordinate]]


def main():
    with open(INPUT_FILE, 'r') as fp:
        lines = [line.rstrip('\n') for line in fp]
    map = parse_input(lines)
    solution = solve(map)
    print(solution)


def solve(map: np.ndarray) -> int:
    neighbors, start_node, goal_node = build_graph(map)
    paths = all_paths(neighbors, start_node, goal_node)
    return max(len(path) for path in paths) - 1


def build_graph(map: np.ndarray) -> tuple[dict[Coordinate, set[Coordinate]], Coordinate, Coordinate]:
    map_height, map_width = map.shape
    start_x = np.where(map[0, :] == '.')[0].item()
    goal_x = np.where(map[-1, :] == '.')[0].item()
    start_node = (0, start_x)
    goal_node = (map_height - 1, goal_x)
    graph = {}
    for y, x in it.product(range(map_height), range(map_width)):
        neighbors = set()
        if map[y, x] == '.':
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
        elif map[y, x] == '<':
            neighbors.add((y, x - 1))
        elif map[y, x] == '>':
            neighbors.add((y, x + 1))
        elif map[y, x] == '^':
            neighbors.add((y - 1, x))
        elif map[y, x] == 'v':
            neighbors.add((y + 1, x))
        if neighbors:
            graph[(y, x)] = neighbors
    return graph, start_node, goal_node


def all_paths(graph: Graph, start: Coordinate, goal: Coordinate) -> Sequence[Sequence[Coordinate]]:
    """Find all simple paths from start to goal in a directed (possibly cyclic) graph.
    
    Args:
        graph: A graph represented as a dictionary of nodes and their neighbors.
        start: The starting node.
        goal: The goal node.
    
    Returns:
        A list of paths from start to goal.
    """
    paths = []
    stack = [[start]]
    while stack:
        path = stack.pop()
        path_end = path[-1]
        new_candidates = [path + [neighbor] for neighbor in graph[path_end] if neighbor not in path]
        for new_path in new_candidates:
            if new_path[-1] == goal:
                paths.append(new_path)
            else:
                stack.append(new_path)
    return paths


def parse_input(lines: Sequence[str]) -> np.ndarray:
    array = np.array([list(line) for line in lines])
    return array


if __name__ == '__main__':
    main()
