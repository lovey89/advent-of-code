#!/usr/bin/env python3

import sys

def addr(regA, regB, regC):
    registers[regC] = registers[regA] + registers[regB]

def addi(regA, valB, regC):
    registers[regC] = registers[regA] + valB

def mulr(regA, regB, regC):
    registers[regC] = registers[regA] * registers[regB]

def muli(regA, valB, regC):
    registers[regC] = registers[regA] * valB

def banr(regA, regB, regC):
    registers[regC] = registers[regA] & registers[regB]

def bani(regA, valB, regC):
    registers[regC] = registers[regA] & valB

def borr(regA, regB, regC):
    registers[regC] = registers[regA] | registers[regB]

def bori(regA, valB, regC):
    registers[regC] = registers[regA] | valB

def setr(regA, ignB, regC):
    registers[regC] = registers[regA]

def seti(valA, ignB, regC):
    registers[regC] = valA

def gtir(valA, regB, regC):
    registers[regC] = int(valA > registers[regB])

def gtri(regA, valB, regC):
    registers[regC] = int(registers[regA] > valB)

def gtrr(regA, regB, regC):
    registers[regC] = int(registers[regA] > registers[regB])

def eqir(valA, regB, regC):
    registers[regC] = int(valA == registers[regB])

def eqri(regA, valB, regC):
    registers[regC] = int(registers[regA] == valB)

def eqrr(regA, regB, regC):
    registers[regC] = int(registers[regA] == registers[regB])

operations = {'addr':addr, 'addi':addi, 'mulr':mulr, 'muli':muli,
              'banr':banr, 'bani':bani, 'borr':borr, 'bori':bori,
              'setr':setr, 'seti':seti, 'gtir':gtir, 'gtri':gtri,
              'gtrr':gtrr, 'eqir':eqir, 'eqri':eqri, 'eqrr':eqrr}

registers = [0,0,0,0,0,0]

def read_data(file_name):
    with open(file_name) as f:
        lines = f.readlines()

    register_declaration, raw_instructions = lines[0], lines[1:]

    ipr = int(register_declaration.split()[1])
    instructions = []

    for instruction in raw_instructions:
        op, a, b, c = instruction.split()
        instructions.append((op, int(a), int(b), int(c)))

    return instructions, ipr

if __name__ == '__main__':
    file_name = sys.argv[1]

    instructions, ipr = read_data(file_name)

    candidates = set()
    i = 0

    print('Note: The program will terminate when the counter reaches 11450. The answer should then be 13775890')

    while registers[ipr] < len(instructions):
        op, a, b, c = instructions[registers[ipr]]

        if registers[ipr] == 28:
            i += 1
            print(i)
            candidate = registers[5]
            if candidate not in candidates:
                candidates.add(candidate)
                last_candidate = candidate
            else:
                print(last_candidate)
                break

        operations[op](a, b, c)

        registers[ipr] +=1

