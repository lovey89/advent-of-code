#!/usr/bin/env python3

import sys

def eliminate_units(sequence):
    stack = []
    for char in sequence:
        popped = False
        if stack:
            peek = stack[-1]
            if abs(char - peek) == 32:
                stack.pop()
                popped = True
        if not popped:
            stack.append(char)
    return len(stack)

if __name__ == '__main__':
    file_name = sys.argv[1]

    with open(file_name) as f:
        raw_sequence = f.read()

    sequence = [ord(c) for c in raw_sequence.rstrip()]

    print(eliminate_units(sequence))
