search pattern
- concentric circles
- row by row scan
- by each ratio



let visited: HashSet<(usize, usize)> = HashSet::new()
let count = 0
for x in 0..w {
  for y in 0..h {
    if let (rx, ry) = reduce_ratio(x, y) {
      if visited.contains((rx,ry)) {
        visited.insert((rx,ry))
        if can_see_asteroid(map, rx, ry, w, h) {
          count += 1
        }
      }
    }
  }
}
count

fn reduce_ratio(x, y) -> Option((usize, usize))
  if y == x && x == 0 return None;
  let m = min(x, y);
  Some(match (x, y, x%m, y%m) {
    (0, _, _, _) => (0, 1)
    (_, 0, _, _) => (1, 0)
    (_, _, y, 0, 0) => (x/m, y/m)
    _ => (x, y)
  })

fn can_see_asteroid(map, rx, ry, w, h)
  let x_iter = (rx..w).step_by(rx)
  let y_iter = (ry..h).step_by(ry)
  x_iter.zip(y_iter).any(|(&x, &y)| map[x][y] == '#')
