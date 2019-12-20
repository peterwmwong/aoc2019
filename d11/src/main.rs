use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::{Add, Mul};

const DEBUG: bool = false;

type Opcode = isize;
type Prog = HashMap<usize, isize>;

pub fn read_prog(prog_str: &str) -> Prog {
    prog_str
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect()
}

trait InputOutput {
    fn input(&mut self) -> isize;
    fn output(&mut self, o: isize);
}

struct ProgramExecution {
    program: Prog,
    pc: usize,
    base: usize,
}

fn opcode(raw: isize) -> Opcode {
    raw % 100
}

#[derive(PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    RelativeToBaseValue,
    RelativeToBaseAddress,
}

#[derive(PartialEq)]
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
    fn new(program: Prog) -> ProgramExecution {
        ProgramExecution {
            program,
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

    fn run(&mut self, io: &mut impl InputOutput) {
        loop {
            match opcode(self.peek(0)) {
                1 => self.op_load_reduce_store(Add::add, "+"),
                2 => self.op_load_reduce_store(Mul::mul, "*"),
                3 => self.op_input(io.input()),
                4 => io.output(self.op_output()),
                5 => self.op_jump_if(|v| v != 0, "not zero"),
                6 => self.op_jump_if(|v| v == 0, "zero"),
                7 => self.op_load_reduce_store(|a, b| if a < b { 1 } else { 0 }, "<"),
                8 => self.op_load_reduce_store(|a, b| if a == b { 1 } else { 0 }, "=="),
                9 => self.op_adjust_relative_base(),
                99 => return,
                n => panic!("Unknown opcode {}", n),
            };
        }
    }
    fn run_once(prog: &Prog, inputs: Vec<isize>) -> Vec<isize> {
        let mut io = RunOnceIO {
            inputs,
            outputs: vec![],
        };
        ProgramExecution::new(prog.to_owned()).run(&mut io);
        io.outputs
    }
}

struct RunOnceIO {
    inputs: Vec<isize>,
    outputs: Vec<isize>,
}

impl InputOutput for RunOnceIO {
    fn input(&mut self) -> isize {
        self.inputs.pop().unwrap()
    }
    fn output(&mut self, o: isize) {
        self.outputs.push(o);
    }
}

#[derive(Copy, Clone, Debug)]
enum OutputState {
    Color,
    Direction,
}

struct RobotIO<'a> {
    robot: &'a mut Robot,
    output_state: OutputState,
}

impl<'a> InputOutput for RobotIO<'a> {
    fn input(&mut self) -> isize {
        *self.robot.panels.get(&self.robot.pos).unwrap_or(&0)
    }
    fn output(&mut self, o: isize) {
        self.output_state = match (self.output_state, o) {
            (OutputState::Color, color) => {
                self.robot.panels.insert(self.robot.pos, color);
                OutputState::Direction
            }
            (OutputState::Direction, dir) => {
                self.robot.rotate(dir);
                self.robot.move_forward();
                OutputState::Color
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Robot {
    panels: HashMap<(isize, isize), isize>,
    pos: (isize, isize),
    dir: Direction,
}

impl Robot {
    fn rotate(&mut self, left_zero_right_one: isize) {
        assert!(left_zero_right_one == 0 || left_zero_right_one == 1);
        self.dir = match (3 + (left_zero_right_one << 1) + self.dir as isize) % 4 {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        };
    }
    fn move_forward(&mut self) {
        let (x, y) = self.pos;
        self.pos = match self.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
    }
    fn run(&mut self, program: &Prog) {
        ProgramExecution::new(program.to_owned()).run(&mut RobotIO {
            robot: self,
            output_state: OutputState::Color,
        });
    }
    fn new() -> Robot {
        Robot {
            panels: HashMap::new(),
            pos: (0, 0),
            dir: Direction::Up,
        }
    }
}

fn draw_panels(panels: &HashMap<(isize, isize), isize>) {
    let (sx, mx, sy, my) = panels
        .keys()
        .fold((0, 0, 0, 0), |(sx, mx, sy, my), &(x, y)| {
            ((min(sx, x), max(mx, x), min(sy, y), max(my, y)))
        });
    for y in sy..=my {
        for x in sx..=mx {
            let v = *panels.get(&(x, y)).unwrap_or(&0);
            print!("{}", if v == 0 { ' ' } else { '*' })
        }
        print!("\n");
    }
}

fn main() {
    let mut r = Robot::new();
    r.panels.insert((0, 0), 1);
    r.run(&read_prog(&read_to_string("./input.txt").unwrap()));
    draw_panels(&r.panels);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_rotate() {
        let test = |dir, left_zero_right_one| {
            let mut robot = Robot::new();
            robot.dir = dir;
            robot.rotate(left_zero_right_one);
            robot.dir
        };
        assert_eq!(test(Direction::Up, 0), Direction::Left);
        assert_eq!(test(Direction::Left, 0), Direction::Down);
        assert_eq!(test(Direction::Down, 0), Direction::Right);
        assert_eq!(test(Direction::Right, 0), Direction::Up);

        assert_eq!(test(Direction::Up, 1), Direction::Right);
        assert_eq!(test(Direction::Left, 1), Direction::Up);
        assert_eq!(test(Direction::Down, 1), Direction::Left);
        assert_eq!(test(Direction::Right, 1), Direction::Down);

        // Verify initial direction
        let robot = Robot::new();
        assert_eq!(robot.dir, Direction::Up);
    }

    #[test]
    fn part1_move_forward() {
        let test = |dir| {
            let mut robot = Robot::new();
            robot.dir = dir;
            robot.move_forward();
            robot.pos
        };
        assert_eq!(test(Direction::Up), (0, -1));
        assert_eq!(test(Direction::Down), (0, 1));
        assert_eq!(test(Direction::Left), (-1, 0));
        assert_eq!(test(Direction::Right), (1, 0));
    }

    #[test]
    fn part1_test0() {
        // A program that output's itself...
        let prog_string = "1102,34915192,34915192,7,4,7,99,0";
        assert_eq!(
            ProgramExecution::run_once(&read_prog(&prog_string), vec![]),
            vec![1219070632396864]
        );
    }

    #[test]
    fn part1() {
        let mut robot = Robot::new();
        robot.run(&read_prog(&read_to_string("./input.txt").unwrap()));
        assert_eq!(robot.panels.len(), 1771);
    }

    #[test]
    fn part2() {
        let mut robot = Robot::new();
        robot.panels.insert((0, 0), 1);
        robot.run(&read_prog(&read_to_string("./input.txt").unwrap()));
        assert_eq!(robot.panels.len(), 249);
    }
}
