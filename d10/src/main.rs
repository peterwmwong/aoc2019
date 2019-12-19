use std::collections::{BTreeMap, HashSet};
use std::f64::consts::PI;
use std::fs::read_to_string;

type Asteroids = HashSet<(isize, isize)>;

fn parse(s: &str) -> Asteroids {
    s.lines()
        .enumerate()
        .map(|(row, row_str)| {
            row_str
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(col, _)| (col as isize, row as isize))
        })
        .flatten()
        .collect()
}

fn angle(&(dx, dy): &(isize, isize)) -> isize {
    let r = (dx as f64).atan2(if dx >= 0 { -dy } else { dy } as f64);
    let r = if dx < 0 { PI - r } else { r };
    (r * 10_f64.powi(10)) as isize
}

fn angle_distance(dx: isize, dy: isize) -> Option<(isize, f32)> {
    if (dx | dy) == 0 {
        return None;
    }
    let dist = ((dx as f32).powi(2) + (dy as f32).powi(2)).sqrt();
    Some((angle(&(dx, dy)), dist))
}

fn visible_asteroids(a: &Asteroids, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
    let mut targets: BTreeMap<isize, (f32, (isize, isize))> = BTreeMap::new();
    for &(ax, ay) in a.iter() {
        if let Some((ang, dist)) = angle_distance(ax - x, ay - y) {
            let entry = (dist, (ax, ay));
            targets
                .entry(ang)
                .and_modify(|e| *e = if e.0 > dist { entry } else { *e })
                .or_insert(entry);
        }
    }
    targets.values().map(|&(_, xy)| xy).collect()
}

fn zap_asteroids(mut a: Asteroids, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
    let mut r = vec![];
    while a.len() > 1 {
        for axy in visible_asteroids(&a, (x, y)) {
            r.push(a.take(&axy).unwrap());
        }
    }
    r
}

fn find_max(a: &Asteroids) -> (isize, isize) {
    *a.iter()
        .max_by_key(|&&xy| visible_asteroids(a, xy).len())
        .unwrap()
}

fn main() {
    let a = parse(&read_to_string("./input.txt").unwrap());
    let m = find_max(&a);
    println!("{:?}", zap_asteroids(a, m)[199]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let a = &parse(&read_to_string("./input.txt").unwrap());
        assert_eq!(visible_asteroids(a, find_max(a)).len(), 230);
    }

    #[test]
    fn part2_test1() {
        let a = parse(&read_to_string("./input-test1.txt").unwrap());
        let m = find_max(&a);
        let asteroids = zap_asteroids(a, m);
        assert_eq!(m, (11, 13));
        for &(i, ex, ey) in &[
            (0, 11, 12),
            (1, 12, 1),
            (2, 12, 2),
            (9, 12, 8),
            (19, 16, 0),
            (49, 16, 9),
            (99, 10, 16),
            (198, 9, 6),
            (199, 8, 2),
            (200, 10, 9),
            (298, 11, 1),
        ] {
            if let Some(&(x, y)) = asteroids.get(i) {
                assert_eq!((x, y), (ex, ey));
            }
        }
    }
}
