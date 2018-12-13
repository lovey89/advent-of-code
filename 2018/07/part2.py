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

def letter_to_time(letter):
    return ord(letter) - ord('A') + 61

if __name__ == '__main__':
    file_name = sys.argv[1]

    available_to_start, grouped_data = read_data(file_name)
    current_time = 0
    work_in_progress = []
    exec_order = []

    while available_to_start or grouped_data:
        available_to_start.sort(reverse=True)
        to_execute = available_to_start.pop()
        exec_order.append(to_execute)
        work_in_progress.append((to_execute, current_time + letter_to_time(to_execute)))

        if not available_to_start and not grouped_data:
            break

        while not available_to_start or len(work_in_progress) == 5:
            work_in_progress.sort(key=operator.itemgetter(1), reverse=True)
            executed, finish_time = work_in_progress.pop()
            current_time = finish_time
        
            for task, dependencies in grouped_data:
                try:
                    dependencies.remove(executed)
                except KeyError:
                    continue
                if not dependencies:
                    available_to_start.append(task)

            grouped_data = [elem for elem in grouped_data if elem[1]]

    max_finish_time = max(work_in_progress, key=operator.itemgetter(1))[1]
    current_time = max_finish_time

    print("".join(exec_order))
    print(current_time)
