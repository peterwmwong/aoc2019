use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::{Add, Mul};

const DEBUG: bool = false;

type Opcode = isize;
type Prog = HashMap<usize, isize>;

#[derive(Debug, Clone)]
struct ProgramExecution {
    program: Prog,
    pc: usize,
    base: usize,
}

fn opcode(raw: isize) -> Opcode {
    raw % 100
}

#[derive(Debug, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    RelativeToBaseValue,
    RelativeToBaseAddress,
}

#[derive(Debug, PartialEq)]
enum ArgType {
    Address,
    Value,
}

fn param_mode(op: isize, argi: usize, arg_mode: &ArgType) -> ParamMode {
    let param_modes = op / 100; // skip over opcode
    let param_mode_code = (param_modes / 10_isize.pow(argi as u32)) % 10;
    match arg_mode {
        ArgType::Address => match param_mode_code {
            0 => ParamMode::Immediate,
            1 => ParamMode::Immediate,
            2 => ParamMode::RelativeToBaseAddress,
            _ => unreachable!(),
        },
        ArgType::Value => match param_mode_code {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::RelativeToBaseValue,
            _ => unreachable!(),
        },
    }
}

impl ProgramExecution {
    fn new(program: &Prog) -> ProgramExecution {
        ProgramExecution {
            program: program.to_owned(),
            pc: 0,
            base: 0,
        }
    }
    fn peek(&mut self, offset: usize) -> isize {
        self.load(self.pc + offset)
    }

    fn next(&mut self) -> isize {
        let v = self.peek(0);
        self.pc += 1;
        v
    }

    fn load(&self, address: usize) -> isize {
        *self.program.get(&address).unwrap_or(&0)
    }

    fn store(&mut self, address: usize, value: isize) {
        *self.program.entry(address).or_insert(0) = value;
    }

    fn relative_to_base_address(&self, offset: isize) -> usize {
        let address = (self.base as isize) + offset;
        assert!(address >= 0);
        address as usize
    }

    fn next_opcode_and_args(&mut self, args: &[ArgType], debug_op_name: &str) -> Vec<isize> {
        let pc = self.pc;
        let op = self.next();
        let result = args
            .iter()
            .enumerate()
            .map(|(i, mode)| match (param_mode(op, i, mode), self.next()) {
                (ParamMode::Position, address) => self.load(address as usize),
                (ParamMode::Immediate, value) => value,
                (ParamMode::RelativeToBaseValue, offset) => {
                    self.load(self.relative_to_base_address(offset))
                }
                (ParamMode::RelativeToBaseAddress, offset) => {
                    self.relative_to_base_address(offset) as isize
                }
            })
            .collect::<Vec<isize>>();
        if DEBUG {
            println!(
                "[{: >3}] {: >4} {}({})",
                pc,
                op,
                debug_op_name,
                result
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        result
    }

    fn op_load_reduce_store<F>(&mut self, reduce: F, debug_op: &'static str)
    where
        F: FnOnce(isize, isize) -> isize,
    {
        let args = self.next_opcode_and_args(
            &[ArgType::Value, ArgType::Value, ArgType::Address],
            &format!("reduce_store[{}]", debug_op),
        );
        let v = reduce(args[0], args[1]);
        if DEBUG {
            println!("       Storing {} into {}", v, args[2]);
        }
        self.store(args[2] as usize, v);
    }

    // TODO: Kinda silly a singly input is passed and used, but this is assumed to
    // become more complicated in the coming days of Advent of Code.
    fn op_input(&mut self, input: isize) {
        let args = self.next_opcode_and_args(&[ArgType::Address], "input");
        assert!(args.len() > 0, "NO INPUT");
        if DEBUG {
            println!("       Loading input {} into {}", input, args[0]);
        }
        self.store(args[0] as usize, input);
    }

    fn op_jump_if<F>(&mut self, f: F, debug_op: &'static str)
    where
        F: FnOnce(isize) -> bool,
    {
        let args = self.next_opcode_and_args(
            &[ArgType::Value, ArgType::Value],
            &format!("jump_if[{}]", debug_op),
        );
        if f(args[0]) {
            if DEBUG {
                println!("       Jumping to {}", args[1]);
            }
            assert!(args[1] >= 0);
            self.pc = args[1] as usize;
        }
    }

    fn op_output(&mut self) -> isize {
        let args = self.next_opcode_and_args(&[ArgType::Value], "output");
        if DEBUG {
            println!("       Output {}", args[0]);
        }
        args[0]
    }

    fn op_adjust_relative_base(&mut self) {
        let args = self.next_opcode_and_args(&[ArgType::Value], "adjust_relative_base");
        let new_base = (self.base as isize) + args[0];
        if DEBUG {
            println!(
                "       Adjusting base {} + {} -> {}",
                self.base, args[0], new_base
            );
        }
        assert!(new_base >= 0);
        self.base = new_base as usize;
    }

    fn run(&mut self, inputs: &mut Vec<isize>) -> Vec<isize> {
        let mut outputs: Vec<isize> = Vec::new();
        loop {
            match opcode(self.peek(0)) {
                1 => self.op_load_reduce_store(Add::add, "+"),
                2 => self.op_load_reduce_store(Mul::mul, "*"),
                3 => self.op_input(inputs.remove(0)),
                4 => outputs.push(self.op_output()),
                5 => self.op_jump_if(|v| v != 0, "not zero"),
                6 => self.op_jump_if(|v| v == 0, "zero"),
                7 => self.op_load_reduce_store(|a, b| if a < b { 1 } else { 0 }, "<"),
                8 => self.op_load_reduce_store(|a, b| if a == b { 1 } else { 0 }, "=="),
                9 => self.op_adjust_relative_base(),
                99 => return outputs,
                n => panic!("Unknown opcode {}", n),
            };
        }
    }

    fn run_once(prog: &Prog, inputs: &mut Vec<isize>) -> Vec<isize> {
        Self::new(prog).run(inputs)
    }
}

pub fn read_prog(prog_str: &str) -> Prog {
    prog_str
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect()
}

fn main() {
    println!(
        "OUTPUT {:?}",
        ProgramExecution::run_once(
            &read_prog(&read_to_string("./input.txt").unwrap()),
            &mut vec![2],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test0() {
        // A program that output's itself...
        let prog = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let prog_string = prog
            .iter()
            .map(isize::to_string)
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(
            ProgramExecution::run_once(&read_prog(&prog_string), &mut vec![]),
            prog
        );
    }

    #[test]
    fn part1_test1() {
        // A program that output's itself...
        let prog_string = "1102,34915192,34915192,7,4,7,99,0";
        assert_eq!(
            ProgramExecution::run_once(&read_prog(&prog_string), &mut vec![]),
            vec![1219070632396864]
        );
    }

    #[test]
    fn part1_test2() {
        // A program that output's itself...
        let prog_string = "104,1125899906842624,99";
        assert_eq!(
            ProgramExecution::run_once(&read_prog(&prog_string), &mut vec![]),
            vec![1125899906842624]
        );
    }

    #[test]
    fn part1() {
        // A program that output's itself...
        assert_eq!(
            ProgramExecution::run_once(
                &read_prog(&read_to_string("./input.txt").unwrap()),
                &mut vec![1]
            ),
            vec![2316632620]
        );
    }
}
