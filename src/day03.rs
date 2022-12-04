use itertools::Itertools;

use std::collections::HashMap;

pub fn a(inp: String) {
    let mut sum = 0usize;
    for line in inp.trim().lines().map(str::as_bytes) {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let c = common(&[first_half, second_half]).unwrap();
        sum += prio(c);
    }

    println!("{}", sum);
}

pub fn b(inp: String) {
    let mut sum = 0usize;

    for (a, b, c) in inp.trim().lines().map(str::as_bytes).tuples() {
        let c = common(&[a, b, c]).unwrap();
        sum += prio(c);
    }

    println!("{}", sum);
}

pub fn common(inputs: &[&[u8]]) -> Option<u8> {
    let mut counts = HashMap::<u8, usize>::new();
    for input in inputs {
        for value in input.iter().unique() {
            counts.entry(*value).and_modify(|i| *i += 1).or_insert(1);
        }
    }

    counts.iter().find(|(_, v)| **v >= inputs.len()).map(|(k, _)| k).copied()
}

fn prio(val: u8) -> usize {
    if val.is_ascii_uppercase() {
        (val - b'A') as usize + 27
    } else {
        (val - b'a') as usize + 1
    }
}
