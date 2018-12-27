#!/usr/bin/env python3

registers = [1,0,0,0,0,0]

registers[4] = ((registers[4] + 2) ** 2) * 19 * 11
registers[3] = (registers[3] + 3) * 22 + 4
registers[4] += registers[3]

if registers[0] == 1:
  registers[3] = (27 * 28 + 29) * 30 * 14 * 32
  registers[4] += registers[3]
  registers[0] = 0

print(sum(x for x in range(1, registers[4] + 1) if registers[4] % x == 0))
