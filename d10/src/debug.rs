// fn debug(
//     a: &Asteroids,
//     t: (isize, isize),
//     angle_to_asteroid: HashMap<(isize, isize), (f32, isize, isize)>,
//     angles: Vec<(isize, isize)>,
// ) {
//     let w = a.iter().max_by_key(|(x, y)| x).unwrap().0;
//     let h = a.iter().max_by_key(|(x, y)| y).unwrap().1;
//     let s = 190;
//     let m: HashMap<(isize, isize), usize> = angles
//         .iter()
//         .map(|angle| angle_to_asteroid.get(angle).unwrap())
//         .enumerate()
//         .skip(s)
//         .take(20)
//         .map(|(i, &(_, x, y))| ((x, y), i + 1))
//         .collect();
//     for y in 0..=h {
//         for x in 0..=w {
//             let c = if let Some(i) = m.get(&(x, y)) {
//                 String::from("X")
//             } else if t.0 == x && t.1 == y {
//                 String::from("0")
//             } else {
//                 String::from(".")
//             };
//             print!("{}", c);
//         }
//         print!("\n");
//     }
// }
