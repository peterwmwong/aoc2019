use std::ops::{Add, Mul};

const DEBUG: bool = false;

type Prog = Vec<isize>;

#[derive(Debug, PartialEq)]
enum ParamMode {
    POSITION,
    IMMEDIATE,
}

fn read_prog(path: &str) -> Prog {
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn param_mode(pc: usize, prog: &Prog, argc: u32) -> ParamMode {
    let param_modes = prog[pc] / 100;
    if 0 == (param_modes / 10_isize.pow(argc - 1)) % 10 {
        return ParamMode::POSITION;
    }
    ParamMode::IMMEDIATE
}

fn arg_value(pc: usize, prog: &Prog, argc: u32, debug: bool) -> isize {
    let value_or_pointer = prog[pc + argc as usize];
    let mode = param_mode(pc, prog, argc);
    let value = match mode {
        ParamMode::POSITION => prog[value_or_pointer as usize],
        ParamMode::IMMEDIATE => value_or_pointer,
    };
    if DEBUG && debug {
        println!(
            "    arg_value({}, prog, {}) {:?} value {}",
            pc, argc, mode, value
        );
    }
    value
}

fn opcode(pc: usize, prog: &Prog) -> isize {
    prog[pc] % 100
}

fn run_op<F: FnOnce(isize, isize) -> isize>(pc: usize, prog: &mut Prog, f: F) -> usize {
    if DEBUG {
        println!(
            "  [{:04} {: >4} {: >4} {: >4}] {:02} - run_op",
            prog[pc],
            prog[pc + 1],
            prog[pc + 2],
            prog[pc + 3],
            opcode(pc, prog),
        );
    }
    let o = prog[pc + 3] as usize;
    prog[o] = f(arg_value(pc, prog, 1, true), arg_value(pc, prog, 2, true));
    pc + 4
}

// TODO: Kinda silly a singly input is passed and used, but this is assumed to
// become more complicated in the coming days of Advent of Code.
fn input_op(pc: usize, prog: &mut Prog, input: isize) -> usize {
    // "The TEST diagnostic program will start by requesting from the user the ID
    // of the system to test by running an input instruction - provide it 1, the
    // ID for the ship's air conditioner unit."
    assert_eq!(param_mode(pc, prog, 1), ParamMode::POSITION);
    let store_address = prog[pc + 1] as usize;
    prog[store_address] = input;
    pc + 2
}

fn output_op(pc: usize, prog: &mut Prog) -> usize {
    if DEBUG {
        println!(
            "  [{:04} {: >4}] {:02} <output_op> ARG1 {: >4}",
            prog[pc],
            prog[pc + 1],
            opcode(pc, prog),
            arg_value(pc, prog, 1, false),
        );
    }
    let a = arg_value(pc, prog, 1, true);
    println!(" > {}", a);
    pc + 2
}

fn run_prog(mut prog: Prog, input: isize) {
    let mut pc = 0;
    loop {
        pc = match opcode(pc, &prog) {
            1 => run_op(pc, &mut prog, Add::add),
            2 => run_op(pc, &mut prog, Mul::mul),
            3 => input_op(pc, &mut prog, input),
            4 => output_op(pc, &mut prog),
            99 => break,
            n => {
                panic!("Unknown opcode {}", n);
            }
        };
    }
}

fn part1(prog: &Prog) {
    println!("part1...");
    run_prog(prog.clone(), 1);
}

fn main() {
    // let prog: Prog = read_prog("../d2/input.txt");
    let prog: Prog = read_prog("./input.txt");
    part1(&prog);
}
