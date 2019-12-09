fn is_valid_part1(n: &u32) -> bool {
    // Work backwards, starting with the least significant.
    // This flips the condition to increasing numbers, from right to left.
    let mut has_double = false;
    let mut prev_d = 10;
    let mut n = *n;
    while n > 0 {
        let d = n % 10;
        if d > prev_d {
            return false;
        }
        has_double |= prev_d == d;
        prev_d = d;
        n = n / 10;
    }
    has_double
}

fn is_valid_part2(n: &u32) -> bool {
    // Work backwards, starting with the least significant.
    // This flips the condition to increasing numbers, from right to left.
    let mut has_double = false;
    let mut same_prev_count = 0;
    let mut prev_d = 10;
    let mut n = *n;
    while n > 0 {
        let d = n % 10;
        if d > prev_d {
            return false;
        }
        if d == prev_d {
            same_prev_count += 1;
        } else {
            has_double |= same_prev_count == 1;
            same_prev_count = 0;
        }

        prev_d = d;
        n = n / 10;
    }
    has_double || same_prev_count == 1
}

fn part1() {
    println!("{:?}", (372037..=905157).filter(is_valid_part1).count());
    println!("{:?}", (372037..=905157).filter(is_valid_part2).count());
}

fn main() {
    part1();
}
