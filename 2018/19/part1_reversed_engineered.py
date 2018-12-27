#!/usr/bin/env python3

registers = [0,0,0,0,0,0]

# Instruction 00 will make the program jump here
registers[4] = ((registers[4] + 2) ** 2) * 19 * 11  # Insts: 17 - 20
registers[3] = (registers[3] + 3) * 22 + 4          # Insts: 21 - 23
registers[4] += registers[3]                        # Inst:  24

if registers[0] == 1:                               # Inst:  25
  registers[3] = (27 * 28 + 29) * 30 * 14 * 32      # Insts: 27 - 32
  registers[4] += registers[3]                      # Inst:  33
  registers[0] = 0                                  # Inst:  34

# Instructions 26 and 35 will make the program jump here
registers[5] = 1                                    # Inst:  01
registers[2] = 1                                    # Inst:  02

while True: # This will represent label L5
  if registers[5] * registers[2] == registers[4]:   # Insts: 03 - 05
    registers[0] += registers[5]                    # Inst:  07

  registers[2] += 1                                 # Inst:  08

  if registers[2] <= registers[4]:                  # Inst:  09. Note that I changed the sign so that I wont have to use else clause
    continue                                        # Inst:  11

  registers[5] += 1                                 # Inst:  12

  if registers[5] <= registers[4]:                  # Inst:  13. Again I have changed the sign
    registers[2] = 1                                # Inst:  02. Can then reuse label L5
    continue                                        # Inst:  15 but changed to jump to L5 instead of L7

  print(registers[0])
  break                                             # Inst:  16
