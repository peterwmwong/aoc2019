use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;

type ConversionReqs = HashMap<String, usize>;
type ConversionsTable = HashMap<String, (usize, ConversionReqs)>;

fn parse(s: &str) -> ConversionsTable {
    s.trim()
        .lines()
        .map(str::trim)
        .map(|s| {
            let (reqs_enc, result_enc) = s.split_at(s.find(" => ").unwrap());
            let result_enc = &result_enc[4..];
            let (result_amt_enc, result) = result_enc.split_at(result_enc.find(' ').unwrap());
            let result_amt = result_amt_enc.parse().unwrap();
            let reqs: ConversionReqs = reqs_enc
                .split(", ")
                .map(|s| s.split_at(s.find(' ').unwrap()))
                .map(|(amt, req)| (req[1..].to_owned(), amt.parse().unwrap()))
                .collect();
            (result[1..].to_owned(), (result_amt, reqs))
        })
        .collect()
}

fn parse_file(file: &'static str) -> ConversionsTable {
    parse(&std::fs::read_to_string(file).unwrap())
}

fn depth_to_ore(t: &ConversionsTable, chem: &str) -> usize {
    if chem == "ORE" {
        return 0;
    }
    let (_, reqs) = t.get(chem).unwrap();
    reqs.keys()
        .map(|chem| depth_to_ore(t, chem) + 1)
        .max()
        .unwrap()
}

// Recursively expand every chemical requirement until all that's left is ORE.
fn find_ore_amount(t: &ConversionsTable, reqs: &mut ConversionReqs) -> usize {
    // Find the most complicated (longest path to ORE) chemical to expand. This allows common chemeicals to coalesce.
    let deepest_chem = &reqs
        .keys()
        .max_by_key(|chem| depth_to_ore(t, chem))
        .unwrap()
        .to_owned();
    let deepest_amt = reqs.remove(deepest_chem).unwrap();
    if deepest_chem == "ORE" {
        return deepest_amt;
    }
    let (amt, chem_reqs) = t.get(deepest_chem).unwrap();
    let mult = (deepest_amt + amt - 1) / amt;
    for (chem, chem_amt) in chem_reqs {
        *reqs.entry(chem.to_owned()).or_insert(0) += chem_amt * mult;
    }
    find_ore_amount(t, reqs)
}

fn ore_amount(t: &ConversionsTable, fuel_amt: usize) -> usize {
    let mut reqs: ConversionReqs = [("FUEL".to_owned(), fuel_amt)].iter().cloned().collect();
    find_ore_amount(&t, &mut reqs)
}

fn find_max(t: &ConversionsTable, max_ore: usize) -> usize {
    // Find max
    let ore_amt_per_fuel = ore_amount(t, 1);
    let mut inc = ore_amount(t, 1000) - ore_amt_per_fuel;
    let mut max = max_ore / ore_amt_per_fuel;
    while ore_amount(t, max) < max_ore {
        max += inc;
    }
    // Binary search
    let mut min = max - inc;
    loop {
        let mid = min + inc / 2;
        match (inc, ore_amount(t, mid).cmp(&max_ore)) {
            (_, Equal) | (1, Less) => return mid,
            (1, Greater) => return max,
            (_, Less) => min = mid,
            (_, Greater) => max = mid,
        }
        inc = max - min;
    }
}

const TRIL: usize = 1000000000000;

fn main() {
    let t = parse_file(&"./input.txt");
    println!("{}", find_max(&t, TRIL));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        fn test(
            t: &ConversionsTable,
            exp_reqs: &[(usize, &str)],
            exp_amt: usize,
            exp_result: &str,
        ) {
            let (amt, reqs) = t.get(exp_result).unwrap();
            assert_eq!(amt, &exp_amt, "Incorrect result amount for {}", exp_result);
            assert_eq!(
                reqs,
                &exp_reqs
                    .iter()
                    .map(|&(amt, req)| (req.to_owned(), amt))
                    .collect()
            );
        }
        let t = parse(
            r"
            10 ORE => 10 A
            1 ORE => 1 B
            7 A, 1 B => 1 C
            7 A, 1 C => 1 D
            7 A, 1 D => 1 E
            7 A, 1 E => 1 FUEL
            ",
        );
        test(&t, &[(7, "A"), (1, "B")], 1, "C");
        test(&t, &[(7, "A"), (1, "C")], 1, "D");
        test(&t, &[(7, "A"), (1, "D")], 1, "E");
        test(&t, &[(7, "A"), (1, "E")], 1, "FUEL");
    }

    #[test]
    fn test_depth_to_ore() {
        let t = parse(
            r"
            10 ORE => 10 A
            1 ORE => 1 B
            7 A, 1 B => 1 C
            7 A, 1 C => 1 D
            7 A, 1 D => 1 E
            7 A, 1 E => 1 FUEL
            ",
        );
        assert_eq!(depth_to_ore(&t, "FUEL"), 5);
        assert_eq!(depth_to_ore(&t, "E"), 4);
        assert_eq!(depth_to_ore(&t, "D"), 3);
        assert_eq!(depth_to_ore(&t, "C"), 2);
        assert_eq!(depth_to_ore(&t, "B"), 1);
        assert_eq!(depth_to_ore(&t, "A"), 1);
        assert_eq!(depth_to_ore(&t, "ORE"), 0);
    }

    #[test]
    fn test_ore_amount() {
        assert_eq!(
            ore_amount(
                &parse(
                    r"
                    10 ORE => 10 A
                    1 ORE => 1 B
                    7 A, 1 B => 1 C
                    7 A, 1 C => 1 D
                    7 A, 1 D => 1 E
                    7 A, 1 E => 1 FUEL
                    ",
                ),
                1
            ),
            31
        );
        assert_eq!(
            ore_amount(
                &parse(
                    r"
                    9 ORE => 2 A
                    8 ORE => 3 B
                    7 ORE => 5 C
                    3 A, 4 B => 1 AB
                    5 B, 7 C => 1 BC
                    4 C, 1 A => 1 CA
                    2 AB, 3 BC, 4 CA => 1 FUEL
                    ",
                ),
                1
            ),
            165
        );
        assert_eq!(
            ore_amount(
                &parse(
                    r"
                    157 ORE => 5 NZVS
                    165 ORE => 6 DCFZ
                    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                    179 ORE => 7 PSHF
                    177 ORE => 5 HKGWZ
                    7 DCFZ, 7 PSHF => 2 XJWVT
                    165 ORE => 2 GPVTF
                    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
                    ",
                ),
                1
            ),
            13312
        );
        assert_eq!(
            ore_amount(
                &parse(
                    r"
                    2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                    17 NVRVD, 3 JNWZP => 8 VPVL
                    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                    22 VJHF, 37 MNCFX => 5 FWMGM
                    139 ORE => 4 NVRVD
                    144 ORE => 7 JNWZP
                    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                    145 ORE => 6 MNCFX
                    1 NVRVD => 8 CXFTF
                    1 VJHF, 6 MNCFX => 4 RFSQX
                    176 ORE => 6 VJHF
                    ",
                ),
                1
            ),
            180697
        );
        assert_eq!(
            ore_amount(
                &parse(
                    r"
                    171 ORE => 8 CNZTR
                    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                    114 ORE => 4 BHXH
                    14 VRPVC => 6 BMBT
                    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                    5 BMBT => 4 WPTQ
                    189 ORE => 9 KTJDG
                    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                    12 VRPVC, 27 CNZTR => 2 XDBXC
                    15 KTJDG, 12 BHXH => 5 XCVML
                    3 BHXH, 2 VRPVC => 7 MZWV
                    121 ORE => 7 VRPVC
                    7 XCVML => 6 RJRHP
                    5 BHXH, 4 VRPVC => 5 LTCX
                    ",
                ),
                1
            ),
            2210736
        );
    }

    #[test]
    fn part1() {
        assert_eq!(ore_amount(&parse_file(&"./input.txt"), 1), 522031);
    }

    #[test]
    fn part2() {
        assert_eq!(find_max(&parse_file(&"./input.txt"), TRIL), 3566577);
    }
}
