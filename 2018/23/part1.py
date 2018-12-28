#!/usr/bin/env python3

import sys
import re
import operator

pattern = re.compile(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)')

def read_data(file_name):
    with open(file_name) as f:
        lines = f.readlines()

    nanobots = []
    for line in lines:
        match = pattern.match(line)
        x,y,z,r = match.group(1,2,3,4)
        nanobots.append(( (int(x), int(y), int(z)), int(r)))

    return nanobots

if __name__ == '__main__':
    file_name = sys.argv[1]

    nanobots = read_data(file_name)

    (sx, sy, sz), sr = max(nanobots, key = operator.itemgetter(1))

    print(sum(1 for (rx, ry, rz), _ in nanobots if abs(rx - sx) + abs(ry - sy) + abs(rz - sz) <= sr))
