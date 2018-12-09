#!/usr/bin/env python3

import sys
import operator
import re
from collections import Counter

pattern = re.compile(r'(\d+), (\d+)')

def create_coordinate_tuple(line):
    match = pattern.match(line)
    x, y = match.group(1, 2)
    return (int(x), int(y))

def normalize_coordinates(coordinates):
    min_x, _ = min(coordinates, key=operator.itemgetter(0))
    _, min_y = min(coordinates, key=operator.itemgetter(1))
    return [(x - min_x, y - min_y) for x, y in coordinates]

if __name__ == '__main__':
    file_name = sys.argv[1]

    with open(file_name) as f:
        raw_coordinates = [create_coordinate_tuple(line) for line in f]

    coordinates = normalize_coordinates(raw_coordinates)

    max_x, _ = max(coordinates, key=operator.itemgetter(0))
    _, max_y = max(coordinates, key=operator.itemgetter(1))

    print(max_x)
    print(max_y)

    grid = [[0 for y in range(max_y + 1)] for x in range(max_x + 1)]
            
    for cx, cy in coordinates:
        x_distance = -cx
        for x in range(max_x + 1):
            y_distance = -cy
            for y in range(max_y + 1):
                current_m_distance = grid[x][y]
                if current_m_distance >= 10000:
                    y_distance += 1
                    continue

                m_distance = abs(x_distance) + abs(y_distance)
                grid[x][y] += m_distance
                y_distance += 1
            x_distance += 1

    number_of_safe_coordinates = 0
    for x in range(max_x + 1):
        for y in range(max_y + 1):
            if grid[x][y] < 10000:
                number_of_safe_coordinates += 1

    print(number_of_safe_coordinates)
