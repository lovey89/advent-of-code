#!/usr/bin/env python3

import sys

def read_data(file_name):
    with open(file_name) as f:
        lines = f.readlines()

    depth = int(lines[0].split()[1])
    tx, ty = lines[1].split()[1].split(',')

    return depth, int(tx), int(ty)

def erosion_level(geologic_index, depth):
    return (geologic_index + depth) % 20183

if __name__ == '__main__':
    file_name = sys.argv[1]

    depth, tx, ty = read_data(file_name)

    first_column = [erosion_level(y * 48271, depth) for y in range(ty + 1)]
    first_column[0] = 0
    erosion_level_grid = [first_column]

    for x in range(1, tx + 1):
        current_column = [erosion_level(x * 16807, depth)]
        for y in range(1, ty + 1):
            current_column.append(erosion_level(current_column[y-1] * erosion_level_grid[x - 1][y], depth))
        erosion_level_grid.append(current_column)
    erosion_level_grid[tx][ty] = erosion_level_grid[0][0]

    type_grid = [[erosion_level % 3 for erosion_level in column] for column in erosion_level_grid]

    print(sum(sum(column) for column in type_grid))
