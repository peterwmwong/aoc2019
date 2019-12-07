use std::ops::{Add, Mul};

fn run_op<F: FnOnce(usize, usize) -> usize>(pc: usize, prog: &mut Vec<usize>, f: F) {
    let a = prog[pc + 1];
    let b = prog[pc + 2];
    let o = prog[pc + 3];
    prog[o] = f(prog[a], prog[b]);
}

fn main() {
    let mut prog: Vec<usize> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // "... replace position 1 with the value 12 and replace position 2 with the value 2"
    prog[1] = 12;
    prog[2] = 2;

    for pc in (0..prog.len()).step_by(4) {
        match prog[pc] {
            1 => run_op(pc, &mut prog, Add::add),
            2 => run_op(pc, &mut prog, Mul::mul),
            99 => break,
            _ => panic!("Unknown opcode"),
        }
    }
    println!("{:?}", prog);
}
