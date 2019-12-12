use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Orbits = HashMap<String, String>;
type Path<'a> = HashSet<&'a str>;

fn read_orbits(path: &str) -> Orbits {
    read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|s| {
            if let [parent, object] = s.split(')').collect::<Vec<&str>>()[..2] {
                // It is assumed (in this limited world) a celestial object cannot
                // orbit more than one other object.
                return (object.to_owned(), parent.to_owned());
            }
            unreachable!()
        })
        .collect()
}

fn path_to_com<'a>(orbits: &'a Orbits, mut obj: &'a str) -> Path<'a> {
    let mut path: Path = Path::new();
    while let Some(parent) = orbits.get(obj) {
        obj = &parent;
        path.insert(obj);
    }
    path
}

fn part1(orbits: &Orbits) {
    let count: usize = orbits
        .keys()
        .map(|object| path_to_com(orbits, object).len())
        .sum();
    println!("Count: {}", count);
}

fn part2(orbits: &Orbits) {
    let you = path_to_com(orbits, "YOU");
    let santa = path_to_com(orbits, "SAN");
    let count = (&you - &santa).union(&(&santa - &you)).count();
    println!("Count: {}", count);
}

fn main() {
    let orbits = read_orbits("./input.txt");
    part1(&orbits);
    part2(&orbits);
}
