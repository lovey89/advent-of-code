#!/usr/bin/env python3

grid_serial = 5719
grid_size = 300

def power(x, y):
    rack_id = x + 10
    return (((rack_id * y + grid_serial) * rack_id) // 100) % 10 - 5

if __name__ == '__main__':
    power_grid = [[power(x + 1, y + 1) for y in range(grid_size)] for x in range(grid_size)]

    max_power = 0

    for x in range(grid_size - 2):
        for y in range(grid_size - 2):
            total_power = sum(power_grid[x][y:y+3] + power_grid[x+1][y:y+3] + power_grid[x+2][y:y+3])
            if total_power > max_power:
                max_power = total_power
                max_power_x = x
                max_power_y = y

    print(max_power, max_power_x + 1, max_power_y + 1) # The grid is off by one hence we need to add 1
