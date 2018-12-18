#!/usr/bin/env python3

grid_serial = 5719
grid_size = 300

def power(x, y):
    #return x + y
    rack_id = x + 10
    return (((rack_id * y + grid_serial) * rack_id) // 100) % 10 - 5

# https://en.wikipedia.org/wiki/Summed-area_table
def create_summed_area_table(power_grid):
    summed_area_table = [[0 for y in range(grid_size + 1)] for x in range(grid_size + 1)]
    for x in range(1, grid_size + 1):
        for y in range(1, grid_size + 1):
            diag =  summed_area_table[x - 1][y - 1]
            left =  summed_area_table[x - 1][y]
            above = summed_area_table[x][y - 1]

            summed_area_table[x][y] = left + above - diag + power_grid[x - 1][y - 1]

    return summed_area_table

def calculate_sum(left, top, s, summed_area_table):
    right = left + s - 1
    bottom = top + s - 1

    area_above = summed_area_table[right][top - 1]
    area_left  = summed_area_table[left - 1][bottom]
    area_diag  = summed_area_table[left - 1][top - 1]

    return summed_area_table[right][bottom] + area_diag - area_above - area_left

if __name__ == '__main__':
    power_grid = [[power(x + 1, y + 1) for y in range(grid_size)] for x in range(grid_size)]

    summed_area_table = create_summed_area_table(power_grid)

    max_power = 0
    for s in range(1, grid_size + 1):
        for x in range(1, grid_size - s + 2):
            for y in range(1, grid_size - s + 2):
                total_power = calculate_sum(x, y, s, summed_area_table)
                if total_power > max_power:
                    max_power = total_power
                    max_power_x = x
                    max_power_y = y
                    max_power_s = s

    print(max_power, max_power_x, max_power_y, max_power_s) # The grid is off by one hence we need to add 1
