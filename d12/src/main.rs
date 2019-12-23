use num_integer::lcm;
use packed_simd::*;

const ZERO: i64x4 = i64x4::splat(0);
const ONE: i64x4 = i64x4::splat(1);
const NEG_ONE: i64x4 = i64x4::splat(-1);

#[derive(Copy, Clone, Debug)]
struct Moon {
    original_pos: i64x4,
    pos: i64x4,
    vel: i64x4,
}

type Moons = [Moon; 4];

fn signum(v: i64x4) -> i64x4 {
    ZERO.gt(v).select(ZERO, ONE) - ZERO.lt(v).select(ZERO, ONE)
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        let pos = i64x4::new(x, y, z, 0);
        Moon {
            original_pos: pos,
            pos,
            vel: ZERO,
        }
    }
    fn apply_gravity(&mut self, other: &mut Moon) {
        let c = signum(self.pos - other.pos);
        self.vel -= c;
        other.vel += c;
    }
    fn apply_vel(&mut self) {
        self.pos += self.vel;
    }
    fn potential_energy(&self) -> i64 {
        ZERO.le(self.pos)
            .select(self.pos, self.pos * NEG_ONE)
            .wrapping_sum()
    }
    fn kinetic_energy(&self) -> i64 {
        ZERO.le(self.vel)
            .select(self.vel, self.vel * NEG_ONE)
            .wrapping_sum()
    }
    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
    fn diff(&self) -> i64x4 {
        (self.pos - self.original_pos).eq(ZERO).select(ZERO, ONE)
            | self.vel.eq(ZERO).select(ZERO, ONE)
    }
}

fn run(moons: &mut Moons) -> i64x4 {
    for i in 0..moons.len() - 1 {
        if let Some((first, elements)) = moons[i..].split_first_mut() {
            for e in elements {
                first.apply_gravity(e);
            }
        }
    }
    moons.iter_mut().fold(i64x4::splat(0), |diff, m| {
        m.apply_vel();
        diff | m.diff()
    })
}

fn total_energy(ms: Moons) -> i64 {
    ms.iter().map(Moon::total_energy).sum()
}

fn find_cycle(mut moons: Moons) -> i64 {
    let mut i = i64x4::splat(0);
    let mut axis_cycles = i64x4::new(0, 0, 0, 1);
    while !axis_cycles.gt(ZERO).all() {
        i += ONE;
        // IF the axis difference is zero and axis_cycle is zero (meaning we haven't found a cycle yet for this axis)
        // THEN save iteration number `i` for that axis
        let cycle_founds = run(&mut moons) | axis_cycles;
        axis_cycles = cycle_founds.eq(ZERO).select(i, axis_cycles);
    }
    lcm(
        axis_cycles.extract(0),
        lcm(axis_cycles.extract(1), axis_cycles.extract(2)),
    )
}

fn main() {
    let moons = [
        Moon::new(-9, 10, -1),
        Moon::new(-14, -8, 14),
        Moon::new(1, 5, 6),
        Moon::new(-19, 7, 8),
    ];
    println!("{}", find_cycle(moons));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simd_signum() {
        assert_eq!(signum(i64x4::splat(7)), ONE);
        assert_eq!(signum(i64x4::splat(-7)), i64x4::splat(-1));
        assert_eq!(signum(ZERO), ZERO);
    }

    #[test]
    fn gravity() {
        let mut a = Moon::new(0, 0, 5);
        let mut b = Moon::new(5, -5, 5);
        a.apply_gravity(&mut b);
        assert_eq!(a.vel, i64x4::new(1, -1, 0, 0));
        assert_eq!(b.vel, i64x4::new(-1, 1, 0, 0));
    }

    #[test]
    fn velocity() {
        let mut a = Moon::new(4, 5, 6);
        a.vel = i64x4::new(1, -3, 0, 0);
        a.apply_vel();
        assert_eq!(a.pos, i64x4::new(5, 2, 6, 0));
    }

    #[test]
    fn energy() {
        let mut a = Moon::new(4, 5, 6);
        a.pos = i64x4::new(1, -8, 0, 0);
        a.vel = i64x4::new(-1, 1, 3, 0);
        assert_eq!(a.potential_energy(), 9);
        assert_eq!(a.kinetic_energy(), 5);
        assert_eq!(a.total_energy(), 45);
    }

    #[test]
    fn part1_test0() {
        let mut moons = [
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        run(&mut moons);
        assert_eq!(moons[0].pos, i64x4::new(2, -1, 1, 0));
        run(&mut moons);
        assert_eq!(moons[0].pos, i64x4::new(5, -3, -1, 0));
    }

    #[test]
    fn part1() {
        let mut moons = [
            Moon::new(-9, 10, -1),
            Moon::new(-14, -8, 14),
            Moon::new(1, 5, 6),
            Moon::new(-19, 7, 8),
        ];
        for _ in 0..1000 {
            run(&mut moons);
        }
        assert_eq!(total_energy(moons), 8538);
    }

    #[test]
    fn part2_test0() {
        let moons = [
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        assert_eq!(find_cycle(moons), 2772);
    }

    #[test]
    fn part2_test1() {
        let moons = [
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];
        assert_eq!(find_cycle(moons), 4686774924);
    }

    #[test]
    fn part2() {
        let moons = [
            Moon::new(-9, 10, -1),
            Moon::new(-14, -8, 14),
            Moon::new(1, 5, 6),
            Moon::new(-19, 7, 8),
        ];
        assert_eq!(find_cycle(moons), 506359021038056);
    }
}
