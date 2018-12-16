#!/usr/bin/env python3

import sys

class TreeNode:
    def __init__(self, sequence):
        (self.no_of_children, self.no_of_meta), sequence = sequence[:2], sequence[2:]
        self.children = []

        for _ in range(self.no_of_children):
            n = TreeNode(sequence)
            self.children.append(n)
            sequence = sequence[n.get_size():]
        self.meta = sequence[:self.no_of_meta]

    def get_size(self):
        return 2 + self.no_of_meta + sum(child.get_size() for child in self.children)

    def sum(self):
        return sum(self.meta + [child.sum() for child in self.children])

def read_data(file_name):
    data = []
    with open(file_name) as f:
        raw = f.read()

    return [int(i) for i in raw.split()]

if __name__ == '__main__':
    file_name = sys.argv[1]

    input_sequence = read_data(file_name)

    node = TreeNode(input_sequence)

    print(node.sum())
