use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day8.prod");
    let grid = input.lines().map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect()).collect();

    let part_one_result = part_one(&grid);
    let part_two_result = part_two(&grid);


    println!("Part One Result: {part_one_result}");
    println!("Part Two Result: {part_two_result}");
}

fn part_two(grid: &Vec<Vec<u32>>) -> usize {
    let length = &grid.len();

    let result = (1..length - 1)
        .cartesian_product(1..length - 1)
        .map(|(x, y)| {
            let height = grid[y][x];
            directions(&grid, x, y)
                .iter()
                .map(|dir| {
                    dir
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| dir.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap();

    return result;
}

fn part_one(grid: &Vec<Vec<u32>>) -> usize {
    let length = grid.len();

    let result = (1..length - 1)
        .cartesian_product(1..length - 1)
        .map(|(y, x)| {
            let height = grid[y][x];
            directions(&grid, x, y)
            .iter()
            .map(|dir| dir.iter().all(|h| *h < height))
            .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (length - 1) * 4;
    return result;
}

fn directions(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = &grid[y];
    let col: Vec<u32> = grid.iter().map(|row| row[x]).collect();

    let (left, right) = row.split_at(x);
    let (up, down) = col.split_at(y);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let down = down[1..].to_vec();
    let right = right[1..].to_vec();

    return [up, down, left, right];
}
