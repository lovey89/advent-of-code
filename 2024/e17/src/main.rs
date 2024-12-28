use std::fs;
use std::ops::BitXor;

const INPUT_FILE: &str = "input.txt";
//const INPUT_FILE: &str = "mini.txt";

#[derive(Debug)]
enum Instruction {
    ADV(usize),
    BXL(usize),
    BST(usize),
    JNZ(usize),
    BXC(usize),
    OUT(usize),
    BDV(usize),
    CDV(usize),
}

impl From<(usize, usize)> for Instruction {
    fn from(value: (usize, usize)) -> Self {
        match value {
            (0, oper) => Self::ADV(oper),
            (1, oper) => Self::BXL(oper),
            (2, oper) => Self::BST(oper),
            (3, oper) => Self::JNZ(oper),
            (4, oper) => Self::BXC(oper),
            (5, oper) => Self::OUT(oper),
            (6, oper) => Self::BDV(oper),
            (7, oper) => Self::CDV(oper),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Registers {
    A: usize,
    B: usize,
    C: usize,
}

impl Registers {
    fn get_combo_value(&self, combo_value: usize) -> usize {
        match combo_value {
            v @ 0..=3 => v,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => panic!(),
        }
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        output_buffer: &mut Vec<usize>,
    ) -> Option<usize> {
        match instruction {
            Instruction::ADV(arg) => {
                self.A /= 2_usize.pow(self.get_combo_value(*arg).try_into().unwrap());
            }
            Instruction::BXL(arg) => {
                self.B = self.B.bitxor(arg);
            }
            Instruction::BST(arg) => {
                self.B = self.get_combo_value(*arg) % 8;
            }
            Instruction::JNZ(arg) => {
                if self.A != 0 {
                    return Some(arg / 2);
                }
            }
            Instruction::BXC(_arg) => {
                self.B = self.B.bitxor(self.C);
            }
            Instruction::OUT(arg) => {
                output_buffer.push(self.get_combo_value(*arg) % 8);
            }
            Instruction::BDV(arg) => {
                self.B = self.A / 2_usize.pow(self.get_combo_value(*arg).try_into().unwrap());
            }
            Instruction::CDV(arg) => {
                self.C = self.A / 2_usize.pow(self.get_combo_value(*arg).try_into().unwrap());
            }
        }
        None
    }
}

fn read_input(filename: &str) -> (Registers, Vec<Instruction>) {
    let binding = fs::read_to_string(filename).unwrap();
    let lines = binding.lines().collect::<Vec<_>>();

    let registers = Registers {
        A: lines[0].split_ascii_whitespace().collect::<Vec<_>>()[2]
            .parse()
            .unwrap(),
        B: lines[1].split_ascii_whitespace().collect::<Vec<_>>()[2]
            .parse()
            .unwrap(),
        C: lines[2].split_ascii_whitespace().collect::<Vec<_>>()[2]
            .parse()
            .unwrap(),
    };
    let mut line_split = lines[4].split_ascii_whitespace();
    line_split.next();
    let instructions = line_split
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|c| (c[0].parse().unwrap(), c[1].parse().unwrap()).into())
        .collect();
    (registers, instructions)
}

fn part_1() {
    let (mut registers, instructions) = read_input(INPUT_FILE);
    println!("{registers:?}");
    println!("{instructions:?}");
    let mut instruction_pointer = 0;
    let mut output_buffer = vec![];
    loop {
        if instruction_pointer >= instructions.len() {
            break;
        }
        //println!("{registers:?}");
        let ret =
            registers.execute_instruction(&instructions[instruction_pointer], &mut output_buffer);
        if let Some(ip) = ret {
            instruction_pointer = ip;
        } else {
            instruction_pointer += 1;
        }
    }
    println!(
        "{}",
        output_buffer
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn part_2() {
    todo!();
}

fn main() {
    use std::time::Instant;
    println!("Day 17");
    println!("---------------");
    let now = Instant::now();
    part_1();
    let elapsed = now.elapsed();
    println!("part1 : {elapsed:.2?}");
    let now = Instant::now();
    part_2();
    let elapsed = now.elapsed();
    println!("part2 : {elapsed:.2?}");
    println!("---------------");
}
