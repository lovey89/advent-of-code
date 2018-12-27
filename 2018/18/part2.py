#!/usr/bin/env python3

import sys
from enum import Enum
from collections import Counter

side_length = 50
iterations = 1000000000

class GroundType(Enum):
    open_ground = '.'
    trees = '|'
    lumberyard = '#'

    def __init__(self, symbol):
        self.symbol = symbol

    @classmethod
    def from_symbol(cls, symbol):
        for ground_type in GroundType:
            if ground_type.symbol == symbol:
                return ground_type

def print_area(area):
    for line in area:
        print(''.join(char.symbol for char in line))
    print()

def read_data(file_name):
    area = [[GroundType.open_ground for x in range(side_length + 2)] for y in range(side_length + 2)]
    with open(file_name) as f:
        lines = f.readlines()

    for row, line in enumerate(lines):
        for col, char in enumerate(line.strip()):
            area[row + 1][col + 1] = GroundType.from_symbol(char)

    return tuple(tuple(row) for row in area)

def next_gen_coord(y, x, area):
    current_ground_type = area[y][x]

    adjecent = area[y - 1][x - 1:x + 2] + area[y + 1][x - 1:x + 2] + (area[y][x - 1],) + (area[y][x + 1],)

    if current_ground_type == GroundType.open_ground:
        return GroundType.trees if adjecent.count(GroundType.trees) >= 3 else current_ground_type
    elif current_ground_type == GroundType.trees:
        return GroundType.lumberyard if adjecent.count(GroundType.lumberyard) >= 3 else current_ground_type
    elif current_ground_type == GroundType.lumberyard:
        return GroundType.lumberyard if GroundType.lumberyard in adjecent and GroundType.trees in adjecent else GroundType.open_ground

def create_next_gen_area(current_area):
    next_gen_area = [[GroundType.open_ground for x in range(side_length + 2)] for y in range(side_length + 2)]
    for y in range(1, side_length + 1):
        for x in range(1, side_length + 1):
            next_gen_area[y][x] = next_gen_coord(y, x, current_area)
    return tuple(tuple(row) for row in next_gen_area)

if __name__ == '__main__':
    file_name = sys.argv[1]

    current_area = read_data(file_name)

    previous_areas = [current_area]
    iteration = 1

    while True:
        next_gen_area = create_next_gen_area(current_area)
        if (next_gen_area in previous_areas):
            # The generated area has been seen before. This pattern will go on forever
            last = previous_areas.index(next_gen_area)
            period = iteration - last
            break
        previous_areas.append(next_gen_area)
        current_area = next_gen_area
        iteration += 1

    area = previous_areas[(iterations - last) % period + last]
    print_area(area)

    flat_map = [coord for row in area for coord in row]

    counter = Counter(flat_map)
    print(counter[GroundType.trees] * counter[GroundType.lumberyard])
