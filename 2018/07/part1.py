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
    available_from_start = set(a) - set(b)
    data.sort(key=operator.itemgetter(1))
    grouped_data = [[k, {x[0] for x in g}] for k, g in groupby(data, key=operator.itemgetter(1))]
    return available_from_start, grouped_data

if __name__ == '__main__':
    file_name = sys.argv[1]

    available_from_start, grouped_data = read_data(file_name)
    print(available_from_start)
    print(grouped_data)
