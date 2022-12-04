pub fn a(inp: String) {
    let overlaps: usize = inp.trim().lines().map(parse_range).map(|(r1, r2)| total_overlaps(r1, r2)).sum();
    println!("{}", overlaps);
}

pub fn b(inp: String) {
    let overlaps: usize = inp.trim().lines().map(parse_range).map(|(r1, r2)| partial_overlaps(r1, r2)).sum();
    println!("{}", overlaps);
}

fn parse_range(inp: &str) -> ((usize, usize), (usize, usize)) {
    let (s1, s2) = inp.split_once(",").unwrap();
    let (s1_begin, s1_end) = s1.split_once("-").unwrap();
    let (s2_begin, s2_end) = s2.split_once("-").unwrap();
    ((s1_begin.parse().unwrap(), s1_end.parse().unwrap()), (s2_begin.parse().unwrap(), s2_end.parse().unwrap()))
}

fn total_overlaps(r1: (usize, usize), r2: (usize, usize)) -> usize {
    if (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1) {
        1
    } else {
        0
    }
}

fn partial_overlaps(r1: (usize, usize), r2: (usize, usize)) -> usize {
    if (r1.0 <= r2.0 && r1.1 >= r2.0)
        || (r1.0 <= r2.1 && r1.1 >= r2.1)
        || (r2.0 <= r1.0 && r2.1 >= r1.0)
        || (r2.0 <= r1.1 && r2.1 >= r1.1)
    {
        1
    } else {
        0
    }
}
