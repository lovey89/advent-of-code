#!/usr/bin/env python3

import sys
import re

file = sys.argv[1]

pattern = re.compile(r'.*@ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)')

cloth = [[0 for x in range(1000)] for y in range(1000)]

overlapping_pieces = 0

pieces = []

with open(file) as f:
    for line in f:
        match = pattern.match(line)
        x, y, width, height = match.group(1,2,3,4)
        pieces.append((int(x), int(y), int(width), int(height)))

for left, top, width, height in pieces:
    for x in range(left, left + width):
        cloth_column = cloth[x]
        for y in range(top, top + height):
            coordinate = cloth_column[y]
            if (coordinate == 0):
                cloth_column[y] = 1
            elif (coordinate == 1):
                overlapping_pieces += 1
                cloth_column[y] = 2

print(overlapping_pieces)
