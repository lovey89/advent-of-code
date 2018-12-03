#!/usr/bin/env python3

import sys

def remove_char_at_index(string, index):
    return string[:index] + string[index+1:]

file = sys.argv[1]

with open(file) as f:
    lines = f.readlines()

length_of_list = len(lines)
length_of_string = len(lines[0])

for i in range(0, length_of_string):
    modified_lines = [remove_char_at_index(x, i) for x in lines]
    for j in range(0, length_of_list):
        string0 = modified_lines[j]
        for k in range(j + 1, length_of_list):
            string1 = modified_lines[k]
            if (string0 == string1):
                print(string0)
                exit(0)
