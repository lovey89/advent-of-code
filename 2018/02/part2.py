#!/usr/bin/env python3

import sys

file = sys.argv[1]

with open(file) as f:
    lines = f.readlines()

length_of_list = len(lines)
length_of_string = len(lines[0])

for i in range(0, length_of_string):
    for j in range(0, length_of_list):
        raw_string0 = lines[j]
        string0 = raw_string0[:i] + raw_string0[i+1:]
        for k in range(j + 1, length_of_list):
            raw_string1 = lines[k]
            string1 = raw_string1[:i] + raw_string1[i+1:]
            if (string0 == string1):
                print(string0)
                exit(0)
