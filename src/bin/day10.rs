use std::str::Lines;

#[derive(Debug, Hash)]
struct CycleSegment {
    num: i32,
    x_val: i32,
}

fn main() {
    let lines = include_str!("../../inputs/day10.prod").lines();

    let part_one_solution = part_one(lines);
    let part_two_solution = part_two(lines);

    println!("Part One: {part_one_solution}");
    println!("Part Two: {part_two_solution}");

}

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: u32 = 3;

fn part_two(lines: :Lines) {
    let mut x = 1;
    let mut cycle = 1;
    let mut screen = [' '; COLS * ROWS];

    for line in lines {
        screen[cycle - 1] = get_pixel(cycle, x);
        cycle += 1;

        if let Some(("addx", num)) = line.split_once(' ') {
            screen[cycle - 1] = get_pixel(cycle, x);
            cycle += 1;
            let num: i32 = num.parse().unwrap();
            x += num;
        }
        return screen
            .chunks(COLS)
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join('/n');
    }
}

fn get_pixel(cycle: usize, x: i32) -> char {
    let curr_col = (cycle - 1) % COLS;
    if(curr_col as i32).abs_diff(x) <= SPRITE_WIDTH / 2 {
        return '*';
    } else {
        return '.';
    }
}

fn part_one(lines: Lines) -> i32 {
    let mut cycle_count = 0;
    let mut x_reg_total = 1;
    //let mut total_count = 0;
    let mut cycle_vals = Vec::new();

    for line in lines {
        //total_count += 1;
        //println!("Processing: {line} - count {total_count}");
        //println!("Cycle Count: {cycle_count}, Register Total: {x_reg_total}");

        if line.starts_with("noop") {
            //immediate increment of counter
            cycle_count += 1;

            //handle the end of the cycle and reset counters as needed
            if handle_end_of_cycle(cycle_count, x_reg_total, &mut cycle_vals) {
                cycle_count = 0;
            }

        } else if line.starts_with("addx") {
            //immediate increment of counter
            cycle_count += 1;
            if handle_end_of_cycle(cycle_count, x_reg_total, &mut cycle_vals) {
                cycle_count = 0;
            }
            
            //split and parse the numbers for x register
            let parts = line.split_once(' ').unwrap();
            let x_reg: i32 = parts.1.parse().unwrap();

            //handle the first cycle in addx
            cycle_count += 1;
            //handle the end of the cycle and reset counters/total as needed
            if handle_end_of_cycle(cycle_count, x_reg_total, &mut cycle_vals) {
                cycle_count = 0;
            }

            //add to the register total 
            x_reg_total += x_reg;
        }

    }
    //return the sum of all the segment x_reg values times their cycle number
    let mut grand_total = 0;
    let mut cycle_total = 0;
    for seg in cycle_vals {
        cycle_total += seg.num;
        grand_total += cycle_total * seg.x_val;
        //println!("{grand_total} += {cycle_total} * {:?}", seg.x_val);
    }
    return grand_total;
}

fn handle_end_of_cycle(cycle_count: i32, register_val: i32, cycle_vals: &mut Vec<CycleSegment>) -> bool {
    if cycle_vals.is_empty() && cycle_count == 20 {
        //println!("Handling count 20...");
        //println!("CycleSegment: {cycle_count}, {register_val}");
        let segment = CycleSegment { num: cycle_count, x_val: register_val};
        cycle_vals.push(segment);
        return true;
    }  

    if cycle_count % 40 == 0 {
        //println!("Handling count {cycle_count} ...");
        //println!("CycleSegment: {cycle_count}, {register_val}");
        let segment = CycleSegment { num: cycle_count, x_val: register_val};
        cycle_vals.push(segment);
        return true;
    }
    
    return false;
}



