#!/usr/bin/env python3

import sys
import re

file = sys.argv[1]

pattern = re.compile(r'#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)')

cloth = [[0 for x in range(1000)] for y in range(1000)]

pieces = []

with open(file) as f:
    for line in f:
        match = pattern.match(line)
        piece_id, x, y, width, height = match.group(1,2,3,4,5)
        pieces.append((piece_id, int(x), int(y), int(width), int(height)))

for _, left, top, width, height in pieces:
    for x in range(left, left + width):
        cloth_column = cloth[x]
        for y in range(top, top + height):
            coordinate = cloth_column[y]
            if (coordinate < 2):
                cloth_column[y] += 1

for piece_id, left, top, width, height in pieces:
    coordinates = [(x,y) for x in range(left, left + width) for y in range(top, top + height)]
    for x, y in coordinates:
        if cloth[x][y] != 1:
            break
    else:
        # This part will only be run when the for loop is exhausted, not on break
        print(piece_id)
        exit(0)
