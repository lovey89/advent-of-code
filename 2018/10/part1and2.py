#!/usr/bin/env python3

import sys
import re
import operator

from matplotlib import pyplot as plt
from matplotlib import animation

pattern = re.compile(r'position=<(.*), (.*)> velocity=<(.*), (.*)>')

def read_data(file_name):
    data = []
    with open(file_name) as f:
        for line in f:
            match = pattern.match(line)
            x, y, vx, vy = match.group(1, 2, 3 ,4)
            # Negate y axis or the letters will be upside down in matplotlib
            data.append((int(x), -int(y), int(vx), -int(vy)))
    return data

def coords_at_time(input_data, time):
    updated_coords = [(x + time * vx, y + time * vy) for x, y, vx, vy in input_data]
    min_x = min(updated_coords, key=operator.itemgetter(0))[0]
    max_x = max(updated_coords, key=operator.itemgetter(0))[0]
    min_y = min(updated_coords, key=operator.itemgetter(1))[1]
    max_y = max(updated_coords, key=operator.itemgetter(1))[1]
    return updated_coords, min_x, max_x, min_y, max_y

if __name__ == '__main__':
    file_name = sys.argv[1]
    input_data = read_data(file_name)

    # Found this data by testing
    starting_time = 10300
    starting_points, min_x, max_x, min_y, max_y = coords_at_time(input_data, starting_time)

    fig, ax = plt.subplots()
    xs, ys = zip(*starting_points)
    mat, = ax.plot(xs, ys, 'o')

    def animate(i):
        i = i + starting_time
        print(i)
        points, min_x, max_x, min_y, max_y = coords_at_time(input_data, i)
        xs, ys = zip(*points)

        # Adjust axis at every step
        ax.axis([min_x - 10, max_x + 10, min_y - 10, max_y + 10])

        mat.set_data(xs, ys)
        return mat,

    ax.axis([min_x, max_x, min_y, max_y])
    ani = animation.FuncAnimation(fig, animate, interval=400)
    plt.show()
