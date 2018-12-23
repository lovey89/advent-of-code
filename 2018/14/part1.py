#!/usr/bin/env python3

starting_recipes = '37'
puzzle_input = 409551

if __name__ == '__main__':
    recipes = [int(c) for c in starting_recipes]
    worker_idx = [0, 1]

    produces_recipes = 2

    while produces_recipes < puzzle_input + 10:
        worker_recipe = [recipes[i] for i in worker_idx]
        recipes_sum = sum(worker_recipe)
        first_recipe = recipes_sum // 10
        second_recipe = recipes_sum % 10

        if first_recipe != 0:
            recipes.append(first_recipe)
            produces_recipes += 1

        recipes.append(second_recipe)
        produces_recipes += 1
        worker_idx = [(idx + recipes[idx] + 1) % len(recipes) for idx in worker_idx]

    new_recipes = recipes[puzzle_input:puzzle_input + 10]

    print(''.join([str(c) for c in new_recipes]))
