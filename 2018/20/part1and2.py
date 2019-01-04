#!/usr/bin/env python3

# Started with implementing Dijkstra myself but saw that this was solved with
# networkx so I copied and did some modifications to the solution found at:
# https://www.reddit.com/r/adventofcode/comments/a7uk3f/2018_day_20_solutions/ec5y3lm

import sys
import networkx
import operator
#import matplotlib.pyplot as plt

def read_data(file_name):
    with open(file_name) as f:
        data = f.read()

    return data[1:-1]

if __name__ == '__main__':
    file_name = sys.argv[1]
    paths = read_data(file_name)
    
    maze = networkx.Graph()

    starting_point = (0,0)
    pos = {starting_point}  # the current positions that we're building on
    stack = []  # a stack keeping track of (starts, ends) for groups
    starts, ends = {starting_point}, set()  # current possible starting and ending positions

    for c in paths:
        if c == '|':
            # an alternate: update possible ending points, and restart the group
            ends.update(pos)
            pos = starts
        elif c in 'NESW':
            # move in a given direction: add all edges and update our current positions
            direction = {'N': (0,-1), 'E': (1,0), 'S': (0,1), 'W': (-1,0)}[c]
            maze.add_edges_from((p, tuple(map(operator.add, p, direction))) for p in pos)
            pos = {tuple(map(operator.add, p, direction)) for p in pos}
        elif c == '(':
            # start of group: add current positions as start of a new group
            stack.append((starts, ends))
            starts, ends = pos, set()
        elif c == ')':
            # end of group: finish current group, add current positions as possible ends
            pos.update(ends)
            starts, ends = stack.pop()

    # find the shortest path lengths from the starting room to all other rooms
    lengths = networkx.algorithms.shortest_path_length(maze, starting_point)

    # Only plot the small sample input file!
    #networkx.draw(maze)
    #plt.show()
    print('part1:', max(lengths.values()))
    print('part2:', sum(1 for length in lengths.values() if length >= 1000))
