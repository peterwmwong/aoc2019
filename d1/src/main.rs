fn fuel(m: u32) -> u32 { (m / 3).saturating_sub(2) }

fn total_fuel(m: u32) -> u32 {
    if let Some(f) = std::num::NonZeroU32::new(fuel(m)) {
        return f.get() + total_fuel(f.get());
    }
    0
}

fn sum_for_input<F: Fn(u32) -> u32>(f: F) -> u32 {
    std::fs::read_to_string("./input.txt").unwrap()
        .trim()
        .lines()
        .map(move |s| f(s.parse().unwrap()))
        .sum()
}

fn main() {
    println!("part1: {}", sum_for_input(fuel)); // expected: 3474920
    println!("part2: {}", sum_for_input(total_fuel)); // expected: 5209504
}
