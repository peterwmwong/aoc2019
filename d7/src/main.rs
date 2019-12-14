use std::cmp::max;
use std::fs::read_to_string;
use std::ops::RangeBounds;
use std::ops::{Add, Mul};

const DEBUG: bool = true;
const NUM_OF_AMPS: usize = 5;
const MAX_AMP_INDEX: usize = NUM_OF_AMPS - 1;

type Opcode = isize;
type Prog = Vec<isize>;
type PhaseSetting = isize;
type PhaseSettings = [PhaseSetting; NUM_OF_AMPS];

#[derive(Debug, Clone)]
struct ProgramExecution {
    program: Prog,
    pc: usize,
}

fn opcode(raw: isize) -> Opcode {
    raw % 100
}

#[derive(Debug, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
enum ArgMode {
    Immediate,
    Op,
}

fn param_mode(op: isize, argi: usize, arg_mode: &ArgMode) -> ParamMode {
    match arg_mode {
        ArgMode::Immediate => ParamMode::Immediate,
        ArgMode::Op => {
            let param_modes = op / 100; // skip over opcode
            if 0 == (param_modes / 10_isize.pow(argi as u32)) % 10 {
                return ParamMode::Position;
            }
            ParamMode::Immediate
        }
    }
}

impl ProgramExecution {
    fn new(program: &Prog) -> ProgramExecution {
        ProgramExecution {
            program: program.to_owned(),
            pc: 0,
        }
    }
    fn peek(&self, offset: usize) -> isize {
        self.program[self.pc + offset]
    }

    fn next(&mut self) -> isize {
        let v = self.peek(0);
        self.pc += 1;
        v
    }

    fn next_opcode_and_args(
        &mut self,
        args: &[ArgMode],
        debug_op_name: &'static str,
    ) -> Vec<isize> {
        let op = self.next();
        let result = args
            .iter()
            .enumerate()
            .map(|(i, mode)| match (param_mode(op, i, mode), self.next()) {
                (ParamMode::Position, address) => self.program[address as usize],
                (ParamMode::Immediate, value) => value,
            })
            .collect::<Vec<isize>>();
        if DEBUG {
            println!(
                "{: >4} | {: <20} | {}",
                op,
                debug_op_name,
                result
                    .iter()
                    .map(|v| format!("{: >4}", v))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
        result
    }

    fn op_load_reduce_store<F>(&mut self, reduce: F)
    where
        F: FnOnce(isize, isize) -> isize,
    {
        let args = self.next_opcode_and_args(
            &[ArgMode::Op, ArgMode::Op, ArgMode::Immediate],
            "op_load_reduce_store",
        );
        let v = reduce(args[0], args[1]);
        if DEBUG {
            println!("       Storing {} into {}", v, args[2]);
        }
        self.program[args[2] as usize] = v;
    }

    // TODO: Kinda silly a singly input is passed and used, but this is assumed to
    // become more complicated in the coming days of Advent of Code.
    fn op_input(&mut self, input: isize) {
        let args = self.next_opcode_and_args(&[ArgMode::Immediate], "op_input");
        if DEBUG {
            println!("       Loading input {} into {}", input, args[0]);
        }
        self.program[args[0] as usize] = input;
    }

    fn op_jump_if<F>(&mut self, f: F)
    where
        F: FnOnce(isize) -> bool,
    {
        let args = self.next_opcode_and_args(&[ArgMode::Op, ArgMode::Op], "op_jump_if");
        if f(args[0]) {
            if DEBUG {
                println!("       Jumping to {}", args[1]);
            }
            assert!(args[1] >= 0);
            self.pc = args[1] as usize;
        }
    }

    fn op_output(&mut self) -> isize {
        let args = self.next_opcode_and_args(&[ArgMode::Op], "op_output");
        if DEBUG {
            println!("       Outputting {}", args[0]);
        }
        args[0]
    }

    fn run(&mut self, inputs: &mut Vec<isize>) -> Option<isize> {
        loop {
            match opcode(self.peek(0)) {
                1 => self.op_load_reduce_store(Add::add),
                2 => self.op_load_reduce_store(Mul::mul),
                3 => self.op_input(inputs.remove(0)),
                4 => return Some(self.op_output()),
                5 => self.op_jump_if(|v| v != 0),
                6 => self.op_jump_if(|v| v == 0),
                7 => self.op_load_reduce_store(|a, b| if a < b { 1 } else { 0 }),
                8 => self.op_load_reduce_store(|a, b| if a == b { 1 } else { 0 }),
                99 => return None,
                n => panic!("Unknown opcode {}", n),
            };
        }
    }
}

fn run_feedback_amp_prog(prog: &Prog, settings: &PhaseSettings) -> isize {
    let mut amps: Vec<(ProgramExecution, Vec<isize>)> = settings
        .iter()
        .map(|&setting| (ProgramExecution::new(prog), [setting].to_vec()))
        .collect();
    let mut prev_output: isize = 0;
    loop {
        for (prog, inputs) in amps.iter_mut() {
            inputs.push(prev_output);
            match prog.run(inputs) {
                Some(output) => prev_output = output,
                _ => return prev_output,
            }
        }
    }
}

fn for_each_setting<F, R>(r: &R, s: &mut PhaseSettings, i: usize, f: &mut F)
where
    F: FnMut(&PhaseSettings) -> (),
    R: RangeBounds<isize> + Clone + Iterator<Item = isize>,
{
    assert!((0..NUM_OF_AMPS).contains(&i));
    for n in r.clone() {
        if !s[0..i].contains(&n) {
            s[i] = n;
            match i {
                MAX_AMP_INDEX => f(s),
                _ => for_each_setting(r, s, i + 1, f),
            }
        }
    }
}

pub fn max_settings<R>(r: &R, prog: Prog) -> isize
where
    R: RangeBounds<isize> + Clone + Iterator<Item = isize>,
{
    let mut max_out: isize = 0;
    for_each_setting(r, &mut [0; NUM_OF_AMPS], 0, &mut |settings| {
        max_out = max(max_out, run_feedback_amp_prog(&prog, settings));
    });
    max_out
}

pub fn read_prog(prog_str: &str) -> Prog {
    prog_str
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test0() {
        assert_eq!(
            run_feedback_amp_prog(
                &read_prog(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
                &[4, 3, 2, 1, 0]
            ),
            43210
        );
    }

    #[test]
    fn part1_test1() {
        assert_eq!(
            max_settings(
                &(0..=4),
                read_prog(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
            ),
            43210
        );
    }

    #[test]
    fn part1_test2() {
        assert_eq!(
            max_settings(
                &(0..=4),
                read_prog(
                    &"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
                )
            ),
            54321
        );
    }

    #[test]
    fn part2_test1() {
        assert_eq!(
            run_feedback_amp_prog(
                &read_prog(
                    &"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                ),
                &[9,8,7,6,5]
            ),
            139629729
        );
    }

    #[test]
    fn part2_test2() {
        assert_eq!(
            run_feedback_amp_prog(
                &read_prog(
                    &"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                ),
                &[9,7,8,5,6]
            ),
            18216
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            max_settings(&(0..=4), read_prog(&read_to_string("./input.txt").unwrap())),
            92663
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            max_settings(&(5..=9), read_prog(&read_to_string("./input.txt").unwrap())),
            14365052
        );
    }
}
