#!/usr/bin/env python3

import sys

file = sys.argv[1]

sum = 0

with open(file) as f:
    for line in f:
        sum += int(line)

print(sum)
