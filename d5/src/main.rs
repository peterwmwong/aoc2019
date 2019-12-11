use std::fs::read_to_string;
use std::ops::{Add, Mul};

const DEBUG: bool = false;

type Prog = Vec<isize>;

#[derive(Debug, PartialEq)]
enum ParamMode {
    POSITION,
    IMMEDIATE,
}

fn read_prog(path: &str) -> Prog {
    read_to_string(path)
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

// TODO: Refactor Instruction Pointer cursoring (avoid passing pc and program)
// TODO: Refactor debugging opcode and arguments

fn arg_value(pc: usize, prog: &Prog, argc: u32) -> isize {
    let value_or_pointer = prog[pc + argc as usize];
    let mode = param_mode(pc, prog, argc);
    let value = match mode {
        ParamMode::POSITION => prog[value_or_pointer as usize],
        ParamMode::IMMEDIATE => value_or_pointer,
    };
    if DEBUG {
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

fn op_load_reduce_store<F>(pc: usize, prog: &mut Prog, reduce: F) -> usize
where
    F: FnOnce(isize, isize) -> isize,
{
    if DEBUG {
        println!(
            "  [{:04} {: >4} {: >4} {: >4}] {:02} - op_load_reduce_store",
            prog[pc],
            prog[pc + 1],
            prog[pc + 2],
            prog[pc + 3],
            opcode(pc, prog),
        );
    }
    let o = prog[pc + 3] as usize;
    prog[o] = reduce(arg_value(pc, prog, 1), arg_value(pc, prog, 2));
    pc + 4
}

// TODO: Kinda silly a singly input is passed and used, but this is assumed to
// become more complicated in the coming days of Advent of Code.
fn op_input(pc: usize, prog: &mut Prog, input: isize) -> usize {
    assert_eq!(param_mode(pc, prog, 1), ParamMode::POSITION);
    let store_address = prog[pc + 1] as usize;
    prog[store_address] = input;
    pc + 2
}

fn op_jump_if<F>(pc: usize, prog: &mut Prog, f: F) -> usize
where
    F: FnOnce(isize) -> bool,
{
    if DEBUG {
        println!(
            "  [{:04} {: >4} {: >4}] {:02} - jump_if_true_op",
            prog[pc],
            prog[pc + 1],
            prog[pc + 2],
            opcode(pc, prog),
        );
    }
    if f(arg_value(pc, prog, 1)) {
        if DEBUG {
            println!("    JUMPING!");
        }
        let arg2 = arg_value(pc, prog, 2);
        assert!(arg2 >= 0);
        return arg2 as usize;
    }
    pc + 3
}

fn op_output(pc: usize, prog: &mut Prog) -> usize {
    if DEBUG {
        println!(
            "  [{:04} {: >4}] {:02} <op_output>",
            prog[pc],
            prog[pc + 1],
            opcode(pc, prog),
        );
    }
    println!(" > {}", arg_value(pc, prog, 1));
    pc + 2
}

fn run_prog(mut prog: Prog, input: isize) {
    let mut pc = 0;
    loop {
        pc = match opcode(pc, &prog) {
            1 => op_load_reduce_store(pc, &mut prog, Add::add),
            2 => op_load_reduce_store(pc, &mut prog, Mul::mul),
            3 => op_input(pc, &mut prog, input),
            4 => op_output(pc, &mut prog),
            5 => op_jump_if(pc, &mut prog, |v| v != 0),
            6 => op_jump_if(pc, &mut prog, |v| v == 0),
            7 => op_load_reduce_store(pc, &mut prog, |a, b| if a < b { 1 } else { 0 }),
            8 => op_load_reduce_store(pc, &mut prog, |a, b| if a == b { 1 } else { 0 }),
            99 => break,
            n => panic!("Unknown opcode {}", n),
        };
    }
}

fn part1(prog: &Prog) {
    println!("Part 1...");
    // "The TEST diagnostic program will start by requesting from the user the
    // ID of the system to test by running an input instruction - provide it 1,
    // the ID for the ship's air conditioner unit."
    run_prog(prog.clone(), 1);
}

fn part2(prog: &Prog) {
    println!("Part 2...");
    // "This time, when the TEST diagnostic program runs its input instruction
    // to get the ID of the system to test, provide it 5, the ID for the ship's
    // thermal radiator controller."
    run_prog(prog.clone(), 5);
}

fn main() {
    let prog: Prog = read_prog("./input.txt");
    part1(&prog);
    part2(&prog);
}
