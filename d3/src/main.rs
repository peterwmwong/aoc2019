use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug)]
enum Ins {
    R(i32),
    U(i32),
    L(i32),
    D(i32),
}

use Ins::*;
type Wire = Vec<Ins>;
type WireMap = HashSet<(i32, i32)>;

fn read_prog() -> Vec<Wire> {
    std::fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|ins_seq_str| {
            ins_seq_str
                .split(',')
                .map(|ins: &str| match ins.split_at(1) {
                    ("R", n @ _) => R(n.parse().unwrap()),
                    ("U", n @ _) => U(n.parse().unwrap()),
                    ("D", n @ _) => D(n.parse().unwrap()),
                    ("L", n @ _) => L(n.parse().unwrap()),
                    _ => panic!("Unknown instruction {}", ins),
                })
                .collect()
        })
        .collect()
}

fn visit(map: &mut WireMap, (x, y): (i32, i32), dx: i32, dy: i32) -> (i32, i32) {
    for nx in min(x, x + dx)..=max(x, x + dx) {
        for ny in min(y, y + dy)..=max(y, y + dy) {
            map.insert((nx, ny));
        }
    }
    (x + dx, y + dy)
}

fn map_wire(w: &Wire) -> WireMap {
    let mut pos = (0, 0);
    let mut map: WireMap = WireMap::new();
    for ins in w {
        pos = match *ins {
            R(n) => visit(&mut map, pos, n, 0),
            U(n) => visit(&mut map, pos, 0, n),
            D(n) => visit(&mut map, pos, 0, -n),
            L(n) => visit(&mut map, pos, -n, 0),
            _ => unreachable!(),
        };
    }
    map
}

fn min_intersection(m1: &WireMap, m2: &WireMap) -> i32 {
    m1.intersection(m2)
        .filter(|(x, y)| x != &0 || y != &0) // Exclude central port
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn debug_map(m: &WireMap) {
    let minx = m.iter().map(|(x, _)| *x).min().unwrap();
    let miny = m.iter().map(|(_, y)| *y).min().unwrap();
    let mut grid = [[b'.'; 1024]; 1024];
    for &(x, y) in m {
        grid[(y - miny) as usize][(x - minx) as usize] = b'*';
    }
    for row in grid.iter().rev() {
        println!("|{}|", String::from_utf8(row.to_vec()).unwrap());
    }
    // wire1_map
    //     .intersection(&wire2_map)
    //     .filter(|(x, y)| x != &0 && y != &0)
    //     .for_each(|(x, y)| println!("({}, {}) -> {}", x, y, x.abs() + y.abs()));
    // println!(
    //     "{:?} ",
    //     wire1_map
    //         .intersection(&wire2_map)
    //         .filter(|(x, y)| x != &0 && y != &0)
    // );
}

fn part1(wires: &Vec<Wire>) {
    let wire1_map = map_wire(&wires[0]);
    let wire2_map = map_wire(&wires[1]);
    println!("{:?} ", min_intersection(&wire1_map, &wire2_map));
}

fn main() {
    let wires = read_prog();
    part1(&wires);
}
