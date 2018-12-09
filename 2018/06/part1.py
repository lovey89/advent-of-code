#!/usr/bin/env python3

import sys
import operator
import re
from collections import Counter

pattern = re.compile(r'(\d+), (\d+)')

def create_coordinate_tuple(index, line):
    match = pattern.match(line)
    x, y = match.group(1, 2)
    return (index, int(x), int(y))

def normalize_coordinates(coordinates):
    _, min_x, _ = min(coordinates, key=operator.itemgetter(1))
    _, _, min_y = min(coordinates, key=operator.itemgetter(2))
    return [(index, x - min_x, y - min_y) for index, x, y in coordinates]

if __name__ == '__main__':
    file_name = sys.argv[1]

    with open(file_name) as f:
        raw_coordinates = [create_coordinate_tuple(index, line) for index, line in enumerate(f)]

    coordinates = normalize_coordinates(raw_coordinates)

    _, max_x, _ = max(coordinates, key=operator.itemgetter(1))
    _, _, max_y = max(coordinates, key=operator.itemgetter(2))

    grid = [[(None, None) for y in range(max_y + 1)] for x in range(max_x + 1)]
    
    for cid, cx, cy in coordinates:
        x_distance = -cx
        for x in range(max_x + 1):
            y_distance = -cy
            for y in range(max_y + 1):
                m_distance = abs(x_distance) + abs(y_distance)
                current_m_distance = grid[x][y][0]
                if current_m_distance == None or m_distance < current_m_distance:
                    grid[x][y] = (m_distance, cid)
                elif m_distance == current_m_distance:
                    grid[x][y] = (m_distance, -1)
                y_distance += 1
            x_distance += 1

    ignore_ids_set = set(coord[1] for coord in grid[0] + grid[-1])
    ignore_ids_set.update(col[0][1] for col in grid)
    ignore_ids_set.update(col[-1][1] for col in grid)

    counter = Counter([grid[x][y][1] for x in range(1, max_x) for y in range(1, max_y) if grid[x][y][1] not in ignore_ids_set])
    maximum = max(counter.items(), key=operator.itemgetter(1))
    print(maximum[1])
