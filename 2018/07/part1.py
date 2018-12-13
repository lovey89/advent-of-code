#!/usr/bin/env python3

import sys
import re
import operator
from itertools import groupby

pattern = re.compile(r'^Step (.).* (.) can begin.')

def read_data(file_name):
    data = []
    with open(file_name) as f:
        for line in f:
            match = pattern.match(line)
            d = match.group(1, 2)
            data.append(d)
    # Reverse zip
    a, b = zip(*data)
    available_from_start = list(set(a) - set(b))
    data.sort(key=operator.itemgetter(1))
    grouped_data = [[k, {x[0] for x in g}] for k, g in groupby(data, key=operator.itemgetter(1))]
    return available_from_start, grouped_data

if __name__ == '__main__':
    file_name = sys.argv[1]

    available_from_start, grouped_data = read_data(file_name)
    exec_order = []

    while available_from_start:
        available_from_start.sort(reverse=True)
        to_execute = available_from_start.pop()
        exec_order.append(to_execute)
        for task, dependencies in grouped_data:
            try:
                dependencies.remove(to_execute)
            except KeyError:
                continue
            if not dependencies:
                available_from_start.append(task)

        grouped_data = [elem for elem in grouped_data if elem[1]]

    print("".join(exec_order))
