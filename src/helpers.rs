use std::collections::HashMap;

pub fn parse_lines<T: std::str::FromStr>(inp: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    inp.trim().lines().map(|v| v.parse::<T>().unwrap()).collect()
}

pub fn parse_csv<T>(inp: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    inp.trim().split(',').map(|v| v.parse::<T>().unwrap()).collect()
}

pub fn parse_single_digit_grid<T: From<u32>>(inp: &str) -> Vec<Vec<T>> {
    inp.lines().map(|row| row.chars().map(|col| T::from(col.to_digit(10).unwrap())).collect()).collect::<Vec<Vec<T>>>()
}

const HORIZ_VERT: [(i8, i8); 4] = [
    (-1, 0), // up
    (0, -1), // left
    (0, 1),  // right
    (1, 0),  // down
];

const DIAGS: [(i8, i8); 8] = [
    (-1, -1), // up-left
    (-1, 0),  // up
    (-1, 1),  // up-right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // down-left
    (1, 0),   // down
    (1, 1),   // down-right
];

#[allow(dead_code)]
pub fn neighbors(
    row: usize,
    col: usize,
    height: usize,
    width: usize,
    diags: bool,
) -> impl Iterator<Item = (usize, usize)> {
    let directions = if diags {
        &DIAGS as &[(i8, i8)]
    } else {
        &HORIZ_VERT as &[(i8, i8)]
    };

    directions.iter().cloned().filter_map(move |(r, c)| {
        let rx = row as i64 + r as i64;
        let cx = col as i64 + c as i64;
        if rx < 0 || cx < 0 {
            return None;
        }
        let rx = rx as usize;
        let cx = cx as usize;
        if rx >= height || cx >= width {
            return None;
        }

        Some((rx, cx))
    })
}

#[allow(dead_code)]
pub fn print_bits(v: u8) -> String {
    let mut s = String::new();
    for i in (0..8).rev() {
        if i == 3 {
            s.push('_');
        }
        if v & (1 << i) != 0 {
            s.push('1');
        } else {
            s.push('0');
        }
    }
    return s;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(print_bits(0b0000_0001), "0000_0001");
        assert_eq!(print_bits(0b0000_0010), "0000_0010");
        assert_eq!(print_bits(0b0000_0100), "0000_0100");
        assert_eq!(print_bits(0b0000_1000), "0000_1000");
        assert_eq!(print_bits(0b0001_0000), "0001_0000");
        assert_eq!(print_bits(0b0010_0000), "0010_0000");
        assert_eq!(print_bits(0b0100_0000), "0100_0000");
    }

    #[test]
    fn test_neighbors() {
        // Normal case, all values work:
        assert_eq!(
            neighbors(1, 1, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (0, 0), // up-left
                (0, 1), // up
                (0, 2), // up-right
                (1, 0), // left
                (1, 2), // right
                (2, 0), // down-left
                (2, 1), // down
                (2, 2), // down-right
            ]
        );

        // Edge case: top-left corner
        assert_eq!(
            neighbors(0, 0, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (0, 1), // right
                (1, 0), // down
                (1, 1), // down-right
            ]
        );

        // Edge case: left side
        assert_eq!(
            neighbors(1, 0, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (0, 0), // up
                (0, 1), // up-right
                (1, 1), // right
                (2, 0), // down
                (2, 1), // down-right
            ]
        );

        // Edge case: bottom-left corner
        assert_eq!(
            neighbors(9, 0, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (8, 0), // up
                (8, 1), // up-right
                (9, 1), // right
            ]
        );

        // Edge case: top-right corner
        assert_eq!(
            neighbors(0, 9, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (0, 8), // left
                (1, 8), // down-left
                (1, 9), // down
            ]
        );

        // Edge case: right side
        assert_eq!(
            neighbors(1, 9, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (0, 8), // up-left
                (0, 9), // up
                (1, 8), // left
                (2, 8), // down-left
                (2, 9), // down
            ]
        );

        // Edge case: bottom-right corner
        assert_eq!(
            neighbors(9, 9, 10, 10, true).collect::<Vec<(usize, usize)>>(),
            vec![
                (8, 8), // up-left
                (8, 9), // up
                (9, 8), // left
            ]
        );
    }
}

fn cmp_maps<K, V>(h1: &HashMap<K, V>, h2: &HashMap<K, V>)
where
    K: Eq + std::hash::Hash + std::fmt::Display,
    V: Eq + std::fmt::Display,
{
    let mut failed = false;
    for k in h1.keys() {
        if !h2.contains_key(k) {
            println!("h2 does not have key: {}", k);
        }
        failed = true;
    }
    for k in h2.keys() {
        if !h1.contains_key(k) {
            println!("h1 does not have key: {}", k);
        }
        failed = true;
    }

    if failed {
        return;
    }

    for (k, v1) in h1 {
        let v2 = h2.get(k).unwrap();
        if v1 != v2 {
            println!("{} differs: {} {}", k, v1, v2);
        }
    }
}

fn print_hashmap<K, V>(h: &HashMap<char, usize>)
where
    K: std::fmt::Display,
    V: std::fmt::Display,
{
    let mut keys = h.keys().cloned().collect::<Vec<char>>();
    keys.sort();
    for key in &keys {
        let count = h.get(key).unwrap();
        print!(" {}:{}", key, count);
    }
    println!();
}
