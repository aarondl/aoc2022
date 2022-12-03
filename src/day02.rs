pub fn a(inp: String) {
    println!("{}", inp.trim().lines().map(round_value_a).sum::<usize>());
}

pub fn b(inp: String) {
    println!("{}", inp.trim().lines().map(round_value_b).sum::<usize>());
}

fn round_value_a(inp: &str) -> usize {
    match inp {
        // them us => our_choice_points + outcome_points
        "A X" => 1 + 3, // rock
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,

        "B X" => 1 + 0, // paper
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 1 + 6, // scissors
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => panic!(),
    }
}

fn round_value_b(inp: &str) -> usize {
    match inp {
        "A X" => 3 + 0, // rock
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,

        "B X" => 1 + 0, // paper
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 2 + 0, // scissors
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => panic!(),
    }
}
