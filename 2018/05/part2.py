#!/usr/bin/env python3

import sys

def eliminate_units(sequence, ignore_char):
    upper_case_char = ignore_char
    lower_case_char = ignore_char + 32
    stack = []
    for char in sequence:
        if (char == upper_case_char or char == lower_case_char):
            continue
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
        raw_sequence = f.read().rstrip()

    unique_chars = set(raw_sequence.upper())
    sequence = [ord(c) for c in raw_sequence]

    print(min(eliminate_units(sequence, ord(c)) for c in unique_chars))
