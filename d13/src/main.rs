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
}

const TILE_EMPTY: isize = 0;
const TILE_WALL: isize = 1;
const TILE_BLOCK: isize = 2;
const TILE_PADDLE: isize = 3;
const TILE_BALL: isize = 4;

#[derive(Copy, Clone, Debug)]
enum GameOutputState {
    Initial,
    Y,
    Tile,
    WaitForScore,
    WaitForScore2,
}

struct GameIO<'a> {
    game: &'a mut Game,
    output_state: GameOutputState,
    x: isize,
    y: isize,
}

impl<'a> GameIO<'a> {
    fn new(game: &'a mut Game) -> GameIO<'a> {
        GameIO {
            game,
            output_state: GameOutputState::Initial,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> InputOutput for GameIO<'a> {
    fn input(&mut self) -> isize {
        // Move paddle towards the ball
        (self.game.ball.0 - self.game.paddle.0).signum()
    }
    fn output(&mut self, o: isize) {
        use GameOutputState::*;
        self.output_state = match (self.output_state, o) {
            (Initial, x) => {
                self.x = x;
                if x == -1 {
                    WaitForScore
                } else {
                    Y
                }
            }
            (WaitForScore, p) => {
                assert_eq!(p, 0);
                WaitForScore2
            }
            (WaitForScore2, score) => {
                self.game.score = score;
                Initial
            }
            (Y, y) => {
                self.y = y;
                Tile
            }
            (Tile, tile) => {
                match tile {
                    TILE_BALL => self.game.ball = (self.x, self.y),
                    TILE_PADDLE => self.game.paddle = (self.x, self.y),
                    _ => (),
                }
                self.game.pixels.insert((self.x, self.y), tile);
                draw_pixels(&self.game.pixels);
                Initial
            }
        }
    }
}

struct Game {
    pixels: HashMap<(isize, isize), isize>,
    ball: (isize, isize),
    paddle: (isize, isize),
    score: isize,
}

impl Game {
    fn run(program: &Prog) -> Game {
        let mut game = Game {
            pixels: HashMap::new(),
            ball: (0, 0),
            paddle: (0, 0),
            score: 0,
        };
        ProgramExecution::new(program.to_owned()).run(&mut GameIO::new(&mut game));
        game
    }
}

fn draw_pixels(pixels: &HashMap<(isize, isize), isize>) {
    print!("\x1B[2J"); // Clear screen
    let mut screen = String::with_capacity(40 * 22);
    let (sx, mx, sy, my) = pixels
        .keys()
        .fold((0, 0, 0, 0), |(sx, mx, sy, my), &(x, y)| {
            ((min(sx, x), max(mx, x), min(sy, y), max(my, y)))
        });
    for y in sy..=my {
        for x in sx..=mx {
            screen.push(match *pixels.get(&(x, y)).unwrap_or(&0) {
                TILE_EMPTY => ' ',
                TILE_WALL => '█',
                TILE_BLOCK => '▓',
                TILE_PADDLE => '▄',
                TILE_BALL => '°',
                _ => unreachable!(),
            });
        }
        screen += "\n";
    }
    print!("{}", &screen);
}

fn main() {
    let mut prog = read_prog(&read_to_string("./input.txt").unwrap());
    prog.insert(0, 2);
    println!("score {}", Game::run(&prog).score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let prog = &read_prog(&read_to_string("./input.txt").unwrap());
        let num_block_tiles = Game::run(&prog)
            .pixels
            .values()
            .filter(|&&v| v == TILE_BLOCK)
            .count();
        assert_eq!(num_block_tiles, 173);
    }

    #[test]
    fn part2() {
        let mut prog = read_prog(&read_to_string("./input.txt").unwrap());
        prog.insert(0, 2); // Fake inserting quarters
        assert_eq!(Game::run(&prog).score, 8942);
    }
}
