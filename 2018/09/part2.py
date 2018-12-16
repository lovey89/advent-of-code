#!/usr/bin/env python3

import sys
from collections import deque, defaultdict

def read_data(file_name):
    with open(file_name) as f:
        input = f.read().split()

    return int(input[0]), int(input[6]) * 100

if __name__ == '__main__':
    file_name = sys.argv[1]
    player_count, max_marble = read_data(file_name)

    scores = defaultdict(int)
    # The current marble will be at the end of the list
    circle = deque([0])

    for marble in range(1, max_marble + 1):
        if marble % 23 == 0:
            circle.rotate(7)
            scores[marble % player_count] += marble + circle.pop()
            circle.rotate(-1)
        else:
            circle.rotate(-1)
            circle.append(marble)

    print(max(scores.values()))
