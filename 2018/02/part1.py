#!/usr/bin/env python3

import sys
from collections import Counter

file = sys.argv[1]

found_two = 0
found_three = 0

with open(file) as f:
    for line in f:
        counter = Counter(line)
        occurences = counter.values()
        if 2 in occurences:
            found_two += 1

        if 3 in occurences:
            found_three += 1

print(found_two * found_three)
