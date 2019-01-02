#!/usr/bin/env python3

import sys
from heapq import heappush, heappop

depth = 0

erosion_level_grid = []
type_grid = []

def read_data(file_name):
    with open(file_name) as f:
        lines = f.readlines()

    global depth
    global tx, ty
    depth = int(lines[0].split()[1])
    ttx, tty = lines[1].split()[1].split(',')
    tx, ty = int(ttx), int(tty)

def prepare_column_to_row_number(col, last_row):
    xlen = len(erosion_level_grid)

    if col < xlen:
        column = erosion_level_grid[col]
        ylen = len(column)
        if last_row < ylen: # We're done
            return

    # The data node didn't exist. We need to complement
    if col == 0 and xlen == 0:
        # We need to create the first column as it doesn't exist
        erosion_level = depth % 20183
        column = [erosion_level]
        erosion_level_grid.append(column)
        type_grid.append([erosion_level % 3])
        ylen = 1

    if col == 0:
        # add more values to first column
        new_erosions = [(y * 48271 + depth) % 20183 for y in range(ylen, last_row + 1)]
        column.extend(new_erosions)
        type_grid[0].extend([e % 3 for e in new_erosions])
        return

    # We are not handling the first column so we need to make sure that the previous column is prepared
    prepare_column_to_row_number(col - 1, last_row)

    if col >= xlen:
        # The column doesn't exist but we know that the previous column exist after the recursive call
        erosion_level = (col * 16807 + depth) % 20183
        column = [erosion_level]
        erosion_level_grid.append(column)
        type_grid.append([erosion_level % 3])
        ylen = 1

    for y in range(ylen, last_row + 1):
        previous_column = erosion_level_grid[col - 1]
        column.append((column[y - 1] * previous_column[y] + depth) % 20183)

    # Check for the target coordinate and set the pre determined value
    if col == tx and last_row >= ty:
        column[ty] = depth % 20183

    type_grid[col].extend([e % 3 for e in column[ylen:]])

def get_type_at_coordinate(x, y):
    try:
        return type_grid[x][y]
    except IndexError:
        prepare_column_to_row_number(x, y)
        return type_grid[x][y]

def neighbour_nodes(x, y, tool):
    current_node_type = get_type_at_coordinate(x, y)

    # A neat way to find the complementary tool for the current area. Try out all different
    # alternatives by hand and you will see that it works
    complement_tool = (2 * tool - current_node_type) % 3 # (tool + (tool - current_node_type)) % 3

    neighbours = [((x, y, complement_tool), 7)]

    if x > 0 and get_type_at_coordinate(x - 1, y) != tool:
        neighbours.append( ((x - 1, y, tool), 1) )

    if y > 0 and get_type_at_coordinate(x, y - 1) != tool:
        neighbours.append( ((x, y - 1, tool), 1) )

    if get_type_at_coordinate(x + 1, y) != tool:
        neighbours.append( ((x + 1, y, tool), 1) )

    if get_type_at_coordinate(x, y + 1) != tool:
        neighbours.append( ((x, y + 1, tool), 1) )

    return neighbours

'''
Tools:
  Neither = 0
  Torch = 1
  Climbing gear = 2

It's designed this way so that the region type's number corresponds to the tool which
cannot be used there
'''
if __name__ == '__main__':
    file_name = sys.argv[1]

    read_data(file_name)

    # We could speed up the algorithm by generating a complete matrix before we are looking
    # for the shortest path but I was interested in an algorithm that would not generate a
    # lot of unused data

    # Initiate data that we know we need. More data will be generated as it is needed. This
    # line can be omitted but will affect the performance slightly (and we know that we need
    # all the data anyway)
    get_type_at_coordinate(tx, ty)

    visited_nodes = set()
    queue = [(0, (0,0,1))]

    while True:
        path_len, node = heappop(queue)

        if node not in visited_nodes: # The node is unvisited
            if node == (tx, ty, 1):
                print(path_len)
                break

            visited_nodes.add(node)
            for neighbour, distance in neighbour_nodes(*node):
                if neighbour not in visited_nodes:
                    heappush(queue, (path_len + distance, neighbour))
