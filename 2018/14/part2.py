#!/usr/bin/env python3

import math

starting_recipes = '37'
puzzle_input = 409551

if __name__ == '__main__':
    recipes = [int(c) for c in starting_recipes]
    worker_idx = [0, 1]

    current_number = int(starting_recipes)

    puzzle_input_length = int(math.log10(puzzle_input)) + 1
    mod_number = 10 ** puzzle_input_length

    while True:
        worker_recipe = [recipes[i] for i in worker_idx]
        recipes_sum = sum(worker_recipe)
        first_recipe = recipes_sum // 10
        second_recipe = recipes_sum % 10

        if first_recipe != 0:
            recipes.append(first_recipe)
            current_number = (current_number * 10 + first_recipe) % mod_number
            if current_number == puzzle_input:
                break

        recipes.append(second_recipe)
        current_number = (current_number * 10 + second_recipe) % mod_number
        if current_number == puzzle_input:
            break
        worker_idx = [(idx + recipes[idx] + 1) % len(recipes) for idx in worker_idx]

    print(len(recipes) - puzzle_input_length)
