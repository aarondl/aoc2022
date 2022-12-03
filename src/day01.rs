use itertools::Itertools;

pub fn a(inp: String) {
    println!("{}", calc_maxes(inp, 1));
}

pub fn b(inp: String) {
    println!("{}", calc_maxes(inp, 3));
}

fn calc_maxes(inp: String, take: usize) -> usize {
    let mut sums = Vec::<usize>::new();
    for group in inp.trim().split("\n\n") {
        let mut sum = 0usize;
        for line in group.lines() {
            sum += line.parse::<usize>().unwrap()
        }
        sums.push(sum);
    }

    sums.iter().sorted().rev().take(take).sum()
}

pub fn a_functional(inp: String) {
    let max =
        inp.trim().split("\n\n").map(|s| s.lines().map(|s| s.parse::<usize>().unwrap()).sum::<usize>()).max().unwrap();
    println!("{}", max);
}

pub fn b_functional(inp: String) {
    let max_3 = inp
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<usize>().unwrap()).sum::<usize>())
        .sorted()
        .take(3)
        .sum::<usize>();

    println!("{}", max_3);
}
