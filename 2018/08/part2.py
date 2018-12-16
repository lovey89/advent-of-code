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

    def value(self):
        if (self.no_of_children == 0):
            return sum(self.meta)
        return sum(self.children[r - 1].value() for r in self.meta if r - 1 < self.no_of_children)

def read_data(file_name):
    data = []
    with open(file_name) as f:
        raw = f.read()

    return [int(i) for i in raw.split()]

if __name__ == '__main__':
    file_name = sys.argv[1]

    input_sequence = read_data(file_name)

    node = TreeNode(input_sequence)

    print(node.value())
