#!/usr/bin/env python3

import sys
import operator
import re
from collections import Counter

pattern = re.compile(r'(\d+), (\d+)')

manhattan_limit = 10000

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

    grid = [[0 for y in range(max_y + 1)] for x in range(max_x + 1)]

    number_of_safe_coordinates = 0
    for x in range(max_x + 1):
        for y in range(max_y + 1):
            number_of_safe_coordinates += int(sum(abs(cx - x) + abs(cy - y) for cx, cy in coordinates) < manhattan_limit)

    print(number_of_safe_coordinates)
