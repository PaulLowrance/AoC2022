fn main() {
    let input = include_str!("../../inputs/day6.prod");
    let part_one_solution = part_one(input);
    println!("Part One Solution: {part_one_solution}");
    let part_two_solution = part_two(input);
    println!("Part Two Solution: {part_two_solution}");
}

fn part_two(in_str: &str) -> i32 {
    let mut counter = 0;
    for window in in_str.chars().collect::<Vec<char>>().windows(14) {
        counter += 1;
        match is_str_unique(window) {
            Some((first_idx, second_idx, ch)) => println!("Evaluating '{:?}'", window),
            None => return counter + 13,
        }
    }
    return 0;
}

fn part_one(in_str: &str) -> i32 {
    let mut counter = 0;
    for window in in_str.chars().collect::<Vec<char>>().windows(4) {
        counter += 1;
        match is_str_unique(window) {
            Some((first_idx, second_idx, ch)) => println!("Evaluating '{:?}'", window),
            None => return counter + 3,
        }
    }
    return 0;
}

fn is_str_unique(window: &[char]) -> Option<(usize, usize, &char)> {
    return window.iter()
        .enumerate()
        .find_map(|(first_idx, ch)| {
        window
            .iter()
            .enumerate()
            .skip(first_idx + 1)
            .find(|(_, other)| ch == *other)
            .map(|(second_idx, _)| (first_idx, second_idx, ch))
    });
}
