#!/usr/bin/env python3

import sys

trantab = str.maketrans('.#', '01')

def read_data(file_name):
    next_state_map = ['0'] * 32

    with open(file_name) as f:
        initial_state = f.readline().split()[2]
        f.readline()
        for line in f:
            state, _, result = line.translate(trantab).split()
            next_state_map[int(state, 2)] = result
    return initial_state.translate(trantab), next_state_map

def calculate_sum(state, zero_idx):
    result = 0
    for index, c in enumerate(state):
        if c == '1':
            result += (index - zero_idx)
    return result

if __name__ == '__main__':
    file_name = sys.argv[1]

    initial_state, next_state_map = read_data(file_name)

    current_state = initial_state
    zero_idx = 0
    last_sum = 0

    # After 200 generations we can see that the increase between each generation is fixed
    for gen in range(200):
        zero_idx += 2
        number = 0
        next_state = []

        flower_found = False
        for i in range(-2, len(current_state) + 2):
            if i > len(current_state) - 3:
                number = ((number << 1) & 31)
            else:
                number = ((number << 1) & 31) | int(current_state[i + 2])
            next_ev = next_state_map[number]
            if next_ev == '1':
                flower_found = True

            if flower_found:
                next_state.append(next_ev)
            else:
                zero_idx -= 1
        current_state = ''.join(next_state).rstrip('0')
        current_sum = calculate_sum(current_state, zero_idx)
        diff = current_sum - last_sum
        #print(diff)
        last_sum = current_sum

result = current_sum + (50000000000 - 200) * (diff)

print(result)
