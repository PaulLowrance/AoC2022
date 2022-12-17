use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

fn main() {
    let input = include_str!("../../inputs/day9.prod");

    let mut instructions = Vec::new();
    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return ,
        };
        let distance = distance.parse().ok().unwrap();
        instructions.push(Instruction{direction, distance});
    }

    let part_one_result = part_one(&instructions);
    let part_two_result = part_two(&instructions);

    println!("Part One Solution: {part_one_result}");
    println!("Part Two Solution: {part_two_result}");
}

fn part_two(instructions: &Vec<Instruction>) -> usize {
    let mut start = Coord {x: 0, y:0};
    let mut rope = vec![start; 10];
    let mut seen = HashSet::new();
    seen.insert(start);

    for instruction in instructions {
        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => rope[0].y -= 1,
                Direction::Down => rope[0].y += 1,
                Direction::Left => rope[0].x -= 1,
                Direction::Right => rope[0].x += 1,
            };

            for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                let diff = Coord {
                    x: rope[head_idx].x - rope[tail_idx].x,
                    y: rope[head_idx].y - rope[tail_idx].y,
                };
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    rope[tail_idx].x += diff.x.signum();
                    rope[tail_idx].y += diff.y.signum();
                    if tail_idx == rope.len() - 1 {
                        seen.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }
    return seen.len();
}
fn part_one(instructions: &Vec<Instruction>) -> usize {
    let mut head = Coord {x: 0, y:0};
    let mut tail = Coord {x: 0, y:0};
    let mut seen = HashSet::new();
    seen.insert(tail);

    for instruction in instructions {
        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => head.y -= 1,
                Direction::Down => head.y += 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            };

            let diff = Coord {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            if diff.x.abs() > 1 || diff.y.abs() > 1 {
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                seen.insert(tail);
            }
        }
    }
    return seen.len();
}
