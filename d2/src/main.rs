use std::ops::{Add, Mul};

type Prog = Vec<usize>;

fn read_prog(path: &str) -> Prog {
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn run_op<F: FnOnce(usize, usize) -> usize>(pc: usize, prog: &mut Prog, f: F) {
    let a = prog[pc + 1];
    let b = prog[pc + 2];
    let o = prog[pc + 3];
    prog[o] = f(prog[a], prog[b]);
}

fn run_prog(mut prog: Prog, noun: usize, verb: usize) -> usize {
    prog[1] = noun;
    prog[2] = verb;
    for pc in (0..).step_by(4) {
        match prog[pc] {
            1 => run_op(pc, &mut prog, Add::add),
            2 => run_op(pc, &mut prog, Mul::mul),
            99 => break,
            _ => panic!("Unknown opcode"),
        }
    }
    prog[0]
}

fn part1(prog: &Prog) {
    // "... replace position 1 with the value 12 and replace position 2 with the value 2"
    println!("part1 - answer {}", run_prog(prog.clone(), 12, 2));
}

fn part2(prog: &Prog) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_prog(prog.clone(), noun, verb) == 19690720 {
                let answer = 100 * noun + verb;
                println!("part2 - noun {} verb {} answer {}", noun, verb, answer);
                return;
            }
        }
    }
    panic!("No solution found");
}

fn main() {
    let prog: Prog = read_prog("./input.txt");
    part1(&prog);
    part2(&prog);
}
