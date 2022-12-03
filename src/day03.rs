use itertools::Itertools;

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
    let mut indexes = [0].repeat(inputs.len());

    loop {
        let value = inputs[0][indexes[0]];
        let mut all = true;
        for i in 1..inputs.len() {
            if inputs[i][indexes[i]] != value {
                all = false;
                break;
            }
        }

        if all {
            return Some(value);
        }

        // Increment the next index
        // [0 0] [0 1] [0 2] [1 0] [1 1] [1 2] [2 0] [2 1] [2 2] None
        let mut index = inputs.len() - 1;
        loop {
            indexes[index] += 1;
            if indexes[index] < inputs[index].len() {
                break;
            }

            indexes[index] = 0;
            if index == 0 {
                return None;
            }
            index -= 1;
        }
    }
}

fn prio(val: u8) -> usize {
    if val.is_ascii_uppercase() {
        (val - b'A') as usize + 27
    } else {
        (val - b'a') as usize + 1
    }
}
