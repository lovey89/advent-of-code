#!/usr/bin/env python3

import sys

file = sys.argv[1]

sum = 0
found_elements = set()
found_elements.add(0)

with open(file) as f:
    lines = f.readlines()

index = 0

while True:
    sum += int(lines[index])
    if (sum in found_elements):
        break
    found_elements.add(sum)
    index = (index + 1) % len(lines)

print(sum)
