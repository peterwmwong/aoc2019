use std::fs::read_to_string;

pub fn check(img: &str, w: usize, h: usize) -> usize {
    let layer_size = w * h;
    let [_, ones, twos] = (0..img.len())
        .step_by(layer_size)
        .map(|start| {
            img[start..start + layer_size]
                .bytes()
                .fold([0, 0, 0], |mut hist, ch| {
                    hist[(ch - b'0') as usize] += 1;
                    hist
                })
        })
        .min_by_key(|&[zeros, _, _]| zeros)
        .unwrap();
    ones * twos
}

pub fn render(img: &str, w: usize, h: usize) -> String {
    let layer_size = w * h;
    let mut out = Vec::from(img[0..layer_size].to_string());
    for start in (layer_size..img.len()).step_by(layer_size) {
        let layer = img[start..start + layer_size].bytes();
        for (out_pixel, pixel) in out.iter_mut().zip(layer) {
            if *out_pixel == b'2' && pixel != b'2' {
                *out_pixel = pixel;
            }
        }
    }
    out[..]
        .chunks(w)
        .fold(String::with_capacity(layer_size + h), |acc, r| {
            acc + &String::from_utf8(r.to_vec()).unwrap() + &"\n"
        })
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(
            check(
                &format!(
                    "{}{}{}",
                    format!("{}{}{}", "00", "21", "11"),
                    format!("{}{}{}", "11", "22", "11"),
                    format!("{}{}{}", "01", "12", "21"),
                ),
                2,
                3
            ),
            8
        )
    }

    #[test]
    fn part1() {
        let image = read_to_string("./input.txt").unwrap();
        assert_eq!(check(&image.trim(), 25, 6), 1965);
    }

    #[test]
    fn part2() {
        let image = read_to_string("./input.txt").unwrap();
        assert_eq!(
            render(&image.trim(), 25, 6),
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n",
                "0110011110100100011010001",
                "1001000010101000001010001",
                "1000000100110000001001010",
                "1011001000101000001000100",
                "1001010000101001001000100",
                "0111011110100100110000100",
            )
        );
    }

    #[test]
    fn part2_test1() {
        assert_eq!(
            render(
                &format!(
                    "{}{}{}{}",
                    format!("{}{}", "02", "22"),
                    format!("{}{}", "11", "22"),
                    format!("{}{}", "22", "12"),
                    format!("{}{}", "00", "00"),
                ),
                2,
                2
            ),
            format!("{}\n{}\n", "01", "10"),
        )
    }
}
