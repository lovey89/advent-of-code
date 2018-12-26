#!/usr/bin/env python3

import sys

registers = [0,0,0,0]

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

operations = (addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr)
operation_candidate_list = [list(operations) for i in range(len(operations))]

def read_data(file_name):
    with open(file_name) as f:
        lines, program = f.read().strip().split('\n\n\n\n')

    lines = lines.splitlines()

    samples = []
    index = 0
    while index < len(lines):
        before = lines[index].split(maxsplit = 1)
        op_info = tuple(int(x) for x in lines[index + 1].split())
        after = lines[index + 2].split(maxsplit = 1)
        samples.append((eval(before[1]), eval(after[1]), op_info))
        index += 4

    instructions = []
    for line in program.splitlines():
        op, a, b, c = line.split()
        instructions.append((int(op), int(a), int(b), int(c)))

    return samples, instructions

def set_registers(state):
    global registers
    registers = state[:]

def validate_sample_with_operation(start_state, end_state, a, b, c, operation):
    set_registers(start_state)
    operation(a, b, c)
    return registers == end_state

if __name__ == '__main__':
    file_name = sys.argv[1]

    samples, program_instructions = read_data(file_name)

    res = 0
    for start_state, end_state, (op, a, b, c) in samples:
        new_valid_ops = [operation for operation in operation_candidate_list[op] if validate_sample_with_operation(start_state, end_state, a, b, c, operation)]
        if len(new_valid_ops) == 1:
            only_op = new_valid_ops[0]
            for operation_candidates in operation_candidate_list:
                try: # Sometime we try to remove an already removed element
                    operation_candidates.remove(only_op)
                except ValueError:
                    pass
        operation_candidate_list[op] = new_valid_ops

    set_registers([0, 0, 0, 0])

    for op, a, b, c in program_instructions:
        operation_candidate_list[op][0](a, b, c)

    print(registers[0])
